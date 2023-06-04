from dataclasses import dataclass, field
from datetime import datetime
from functools import cached_property
from pathlib import Path

from instructnest.codegen import jinja_template
from instructnest.common import OPERANDS_TRANSLATED_TYPES_SIZE
from instructnest.instpatern import InstructionPattern
from instructnest.instpatern import OPERANDS_TRANSLATED_NAMES_MAP
from instructnest.decored_minifier import InstructionDecoderDeviceMinimisation
from instructnest.utils import compute_bitmask


PSEUDO_TYPE_MAP = {
    "u8": "u8",
    "u": "u32",
    "c2s8": "i8",
    "c2s32": "i32",
    "c2s": "i32",
}

DATAWORD_SIZED_MAP = {
    "u8": "DataSizeByte",
    "u16": "DataSizeWord",
    "u32": "DataSizeDouble",
    "u64": "DataSizeLong",
    "i8": "DataSizeSignedByte",
    "i16": "DataSizeSignedWord",
    "i32": "DataSizeSignedDouble",
    "i64": "DataSizeSignedLong",
}


PSEUSO_FIELDS = {
    "name": "",
    "flag": "",
    "syntax": ""
}

lazyloaded = object()

@dataclass
class InstructionDecoderGeneratorSettings:
    name_enum_instuctionset: str = "InstructionSet"
    name_macro_bitcoded_values: str = "extract_bitcoded"
    name_enum_default_variant: str = "Invalid"
    name_enum_datawordsized: str = "DataWordSized"
    name_impt_datawordsized: str = "emsgine_lib::models::bytes"
    syntax_indent: str = " " * 4
    word_size = 2
    flag_ignore_enum_fields: bool = True
    flag_from_solution: bool = True
    flag_from_pattens_file: bool = True
    input: str = lazyloaded
    target: str = lazyloaded
    schema: dict[str, str] = lazyloaded


    def update(self, values: dict):
        for key, value in values.items():
            if hasattr(self, key):
                setattr(self, key, value)


class InstructionDecoderGenerator:

    template_dir = Path(__file__).parent.joinpath("templates", "rust", "disassembly")
    decoder_minifier = InstructionDecoderDeviceMinimisation()

    def __init__(self, variable = "", enum_name= "", ignore_enum_fields = True):
        self.variable = variable or "inst"
        self.enum_instuctionset = enum_name or "InstructionSet"
        self.bitcoded_macro_name = "extract_bitcoded"
        self.indent = " " * 4
        self.word_size = 2
        self.value_extractors = {}
        self.ignore_enum_fields = ignore_enum_fields

    @cached_property
    @jinja_template
    def template_math(self):
        return "decoder_match"

    @cached_property
    @jinja_template
    def template_value_extactor_macro(self):
        return "value_extactor_macro"

    @cached_property
    @jinja_template
    def template_decoder_function(self):
        return "decoder_function.rs"

    @cached_property
    @jinja_template
    def template_closure_wordsize(self):
        return "closure_wordsize.rs"

    @cached_property
    @jinja_template
    def template_decoder_terminal(self):
        return "decoder_terminal"

    @cached_property
    @jinja_template
    def template_extract_function(self):
        return "extract_function"

    @cached_property
    @jinja_template
    def template_enum_declaration(self):
        return "enum_declaration"

    @cached_property
    @jinja_template
    def template_impl_mnemonic(self):
        return "impl_mnemonic.rs"

    @cached_property
    @jinja_template
    def template_impl_format_inst(self):
        return "impl_format_inst.rs"

    @cached_property
    def template_data_word_sized(self):
        return f"{self.enum_datawordsized}::" + "{variant}({constructor})"

    @cached_property
    def template_variant_instancing(self):
        return f"{self.enum_instuctionset}::" + "{enum_variant}{enum_contructor}"

    @cached_property
    def template_extract_bitcoded(self):
        return f"{self.bitcoded_macro_name}!({self.variable}, " + "{tt}[{mask}])"

    @property
    def match_default(self):
        return "Err(0u8)"
        # return self.enum_instuctionset + "::Invalid"

    def render_math(self, action, branches, c_indent: int):
        return self.template_math.render(
            action=action,
            variable=self.variable,
            branches=branches,
            m_indent= self.indent * c_indent,
            br_indent= self.indent * (c_indent + 1),
            default=self.match_default
        )


    def render_decoder_terminal(self, name, data_fields: dict, c_indent: int, closure_gates=[]):

        result: str = self.template_decoder_terminal.render(
            closure_gates=closure_gates,
            enum_name=self.enum_instuctionset,
            enum_variant=name,
            fields=data_fields.items()
        )

        return f"\n{self.indent * c_indent}".join(result.splitlines())


    def render_closure_wordsize(self, word_size: int):
        return self.template_closure_wordsize.render(
            variable=self.variable,
            word_size=word_size
        )

    def render_variant_instancing(self, name, data_fields: dict):
        # if self.ignore_enum_fields:
        #     data_fields = {}
        enum_contructor = ", ".join([
            f"{var}: {val}" for var, val in data_fields.items()
        ])
        if enum_contructor:
            enum_contructor = " { " + enum_contructor + " }"

        return self.template_variant_instancing.format(
            enum_variant=name,
            enum_contructor=enum_contructor
        )

    def render_extract_bitcoded(self, mask, type):
        return self.template_extract_bitcoded.format(
            tt=PSEUDO_TYPE_MAP.get(type, type),
            mask=mask
        )

    def render_expr_extract_bits(self, components, capacity):
        cast = None
        if capacity >= self.word_size * 8:
            cast = OPERANDS_TRANSLATED_TYPES_SIZE[capacity // 8 + 1]

        def rnd_line(msk, sft, wrd):
            exp = f"$v[{wrd}]"

            # casting
            if cast:
                exp = f"{exp} as {cast}"

            # apply bitmask
            if msk != hex(self.word_size * 8):
                exp = f"({exp}) & {msk}"

            # shiting
            if sft > 0:
                exp = f"({exp}) >> {sft}"
            if sft < 0:
                exp = f"({exp}) << {-sft}"

            return exp

        return " | ".join([rnd_line(*i) for i in components])

    def render_value_extactor_macro(self):
        return self.template_value_extactor_macro.render(
            macro_name=self.bitcoded_macro_name,
            rules=[(
                item,
                self.render_expr_extract_bits(
                    action["components"],
                    action["capacity"],
                )
            ) for item, action in self.value_extractors.items()]
        )

    def render_decoder_function(self, fn_name, matcher, extract_macro=''):
        return self.template_decoder_function.render(
            function_name=fn_name,
            variable=self.variable,
            enum_name=self.enum_instuctionset,
            matcher=matcher,
            extract_macro=extract_macro,
            dataword_impt=self.impt_datawordsized,
            dataword_enum=self.enum_datawordsized
        )

    def render_enum_declaration(self):
        return self.template_enum_declaration.render(
            enum_name=self.enum_instuctionset,
            enum_variants=[(
                variant,
                "" if self.ignore_enum_fields else ", ".join([f"{v}: {t}" for v, t in vairant_data["fields"].items()]),
                vairant_data["syntax"]
            ) for variant, vairant_data in self.enum_variants.items()]
        )

    def render_impl_instnamespace(self):
        return f"impl InstructionNamespace for {self.enum_instuctionset} " + "{}"

    def render_impl_mnemonic(self):
        return self.template_impl_mnemonic.render(
            enum_name=self.enum_instuctionset,
            enum_variants=[(
                variant,
                vairant_data["mnemonic"]
            ) for variant, vairant_data in self.enum_variants.items()]
        )

    def render_impl_format_inst(self):
        return self.template_impl_format_inst.render(
            enum_name=self.enum_instuctionset,
            enum_variants=[(
                variant,
                vairant_data["formatstr"]
            ) for variant, vairant_data in self.enum_variants.items()]
        )

    def render_module(self, use_statements, decl_blocks):
        return "{}\n\n{}\n".format(
            "\n".join(use_statements),
            "\n\n".join(decl_blocks),
        )

    def convert_branch_key(self, key: str):
        return key.split("::")[1][1:-1]

    def __trace_variant_instaces(self, name: str, data_fields: dict, data_extra: dict = None):
        if False and name in self.enum_variants:
            if self.enum_variants[name] != data_fields:
                raise TypeError("This variant defined but with other operands")

        self.enum_variants[name] = dict(
            extra=data_extra or dict(),
            fields=dict(
                (
                    OPERANDS_TRANSLATED_NAMES_MAP.get(k, k),
                    PSEUDO_TYPE_MAP.get(v, v)
                ) for k,v in data_fields.items()
            )
        )

    def eval_extraction_computation(self, patten_data: InstructionPattern, level) -> dict[str, str]:
        operands: dict[str, dict] = patten_data.data['operands']
        etp = patten_data.data.get('etp', {})

        result = {}

        for k,v in operands['masks'].items():
            new_name = OPERANDS_TRANSLATED_NAMES_MAP.get(k, k)
            constructor=[self.render_extract_bitcoded(v, operands["types"][k])]
            variant=DATAWORD_SIZED_MAP[operands["types"][k]]

            if k in etp:
                ctr = constructor[0]

                need_mut = " | " in etp[k]
                if not need_mut and etp[k].split()[0] not in ['cast']:
                    need_mut = True

                constructor = [f"let mut ext = {ctr};"] if need_mut else [f"let ext = {ctr};"]
                need_end = True
                for index, op in enumerate(etp[k].split(" | ")):
                    op, opr = op.split()

                    last_op = index + 1 == len(etp[k].split(" | "))

                    if op=="offset":
                        constructor.append(
                            f"ext += {opr};"
                        )
                    if op=="mul":
                        constructor.append(
                            f"ext *= {opr};"
                        )
                    if op=="cast":
                        if opr=="c2":
                            capacity = len(operands['operands'][k])
                            bs_ms, *bs_rs = list(range(1, capacity+1))
                            msb = compute_bitmask([bs_ms], capacity=capacity)
                            rsb = compute_bitmask(bs_rs, capacity=capacity)
                            abs = f"-{2 ** (capacity - 1)}"
                            cast_tt = f'i{operands["types"][k][1:]}'
                            cast_op = f"{abs} * (({msb} & ext as {cast_tt}) >> {capacity - 1}) + ({rsb} & ext as {cast_tt})"
                            variant = DATAWORD_SIZED_MAP[cast_tt]
                            if last_op:
                                constructor.append(cast_op)
                                need_end = False
                                continue
                            constructor.append(f"let exp = {cast_op};")

                if need_end:
                    constructor.append("ext")

            str_constructor = constructor[0]
            if len(constructor) > 1:
                str_constructor = "\n".join([f"{self.indent * 4}{i}" for i in constructor])
                str_constructor = "{" + f"\n{str_constructor}\n{self.indent * 3}" + "}"

            result[new_name] = self.template_data_word_sized.format(
                variant=variant,
                constructor=str_constructor
            )

        return result


    def eval_terminal_branch(self, data: str, level: int) -> str:
        patern = InstructionPattern.fromstr(data)

        operands = patern.compute_operands_extract(self.word_size)
        patern.data['operands'] = operands
        self.value_extractors.update(operands["extractors"])

        # compute format str
        str_format = patern.syntax
        opr = [c for c in str_format if c in operands['types']]
        arg_format = []
        for op in opr:
            str_format = str_format.replace(op, "{}")
            arg_format.append(OPERANDS_TRANSLATED_NAMES_MAP.get(op, op))

        self.enum_variants[patern.name] = dict(
            **operands,
            syntax=patern.syntax,
            mnemonic=patern.syntax.split()[0],
            formatstr=(str_format, arg_format)
        )

        ins_param = self.eval_extraction_computation(patern, level)

        closure_gates=[]

        if patern.width > self.word_size:
            closure_gates.append(self.render_closure_wordsize(patern.width))

        return self.render_decoder_terminal(
            name=patern.name,
            data_fields=ins_param,
            c_indent=level,
            closure_gates=closure_gates
        )

        print(imp)
        # [(
        #             OPERANDS_TRANSLATED_NAMES_MAP.get(k, k),
        #             vairant_data["types"][k],
        #             vairant_data["masks"][k]
        #         ) for k in vairant_data["operands"]]

        return self.render_variant_instancing(
            name=patern.name,
            data_fields={} if self.ignore_enum_fields else ins_param
        )

    def eval_pseudo_terminal_branch(self, data: list[str], level: int) -> str:
        return self.eval_terminal_branch(data[0], level) + ", // see..."

    def eval_match_level(self, data: dict, level: int):
        if isinstance(data, str):
            return self.eval_terminal_branch(data, level)
        action = list(data)[0]
        data = data[action]
        if isinstance(data, list):
            return self.eval_pseudo_terminal_branch(data, level)
        action = action.split()[1]
        return self.render_math(
            action=action,
            c_indent=level,
            branches=[(
                self.convert_branch_key(k), self.eval_match_level(data[k], level+1)
            ) for k in data['balance']]
        )

    def render_extract_function(self, extract_macros):
        # OPERANDS_TRANSLATED_NAMES_MAP.get(k, k)
        return self.template_extract_function.render(
            enum_name=self.enum_instuctionset,
            extract_macro=extract_macros,
            macro_name=self.bitcoded_macro_name,
            variable="value",
            enum_variants=[(
                variant,
                [(
                    OPERANDS_TRANSLATED_NAMES_MAP.get(k, k),
                    vairant_data["types"][k],
                    vairant_data["masks"][k]
                ) for k in vairant_data["operands"]],
            ) for variant, vairant_data in self.enum_variants.items()]
        )

    def apply_settings(self, settings: InstructionDecoderGeneratorSettings = None):
        if settings is None:
            return
        for field in settings.__dataclass_fields__:
            ps_type, ps_field = None, field
            if "_" in field:
                ps_type, ps_field = field.split("_", maxsplit=1)

            ps_field = ps_field if ps_type in PSEUSO_FIELDS else field
            setattr(self, ps_field, getattr(settings, field))


    def reset(self):
        self.value_extractors = {}
        self.enum_variants = {}

    def run(self, settings: InstructionDecoderGeneratorSettings, **kwargs):
        settings.update(kwargs)
        self.apply_settings(settings)
        self.reset()

        if settings.flag_from_pattens_file:
            self.input = self.decoder_minifier.run(
                Path(settings.input).read_text(encoding="utf-8").splitlines()
            )

        rs_matcher = self.eval_match_level(self.input['trace'], 1)
        rs_extactor_macro = self.render_value_extactor_macro()
        rs_decoder_function = self.render_decoder_function("decode", rs_matcher, rs_extactor_macro)

        rs_instruction_module = self.render_module(
            use_statements=[
                "use emsgine_lib::models::instructionset::InstructionNamespace;",
                "use emsgine_lib::models::instructionset::MnemonicInstruction;",
                "use emsgine_lib::models::instructionset::FormatInstruction;",
                "use emsgine_lib::models::bytes::DataWordSized;",
                "use emsgine_lib::lookup::Lookup;",
                "use emsgine_lib::lookup::safe_lookup;",
            ],
            decl_blocks=[
                self.render_enum_declaration(),
                self.render_impl_instnamespace(),
                self.render_impl_mnemonic(),
                self.render_impl_format_inst(),
            ]
        )
        # rs_extract_function = self.render_extract_function(rs_extactor_macro)

        rs_generated_tag = f"// generated-code: {datetime.now().isoformat()}\n\n"

        target_dir = Path(settings.target)
        target_dir.joinpath("decoder.rs").write_text(rs_generated_tag + rs_decoder_function)
        target_dir.joinpath("instructions.rs").write_text(rs_generated_tag + rs_instruction_module)
        # target_dir.joinpath("extract.rs").write_text(rs_extract_function)


        # rs_module = rs_enum_declaration + "\n\n" + rs_value_extactor_macro + "\n\n\n" + rs_decoder_function
        # with open(settings.target + r"\result.rs", mode="w", encoding="utf-8") as file:
        #     file.write(rs_module)



if __name__ == "__main__":
    from pathlib import Path
    import json

    filepath = Path(r"D:\dev\world-embeddable-simulator\result.json")

    data = json.loads(filepath.read_text("utf-8"))
    r = InstructionDecoderGenerator().run(data)
    Path("res.rust.txt").write_text(r)