from pathlib import Path
from instructnest.rustgen import InstructionDecoderGenerator
from instructnest.rustgen import InstructionDecoderGeneratorSettings
from instructnest.decored_minifier import InstructionDecoderDeviceMinimisation


# filepath = Path(r")



def main():
    decoder_codegen = InstructionDecoderGenerator()
    decoder_settings = InstructionDecoderGeneratorSettings(
        name_enum_instuctionset="AvrInstructionSet",
        input=r"D:\dev\emsgine\docs\instruction-sets\avr.opcode.nfo",
        target=r"D:\dev\emsgine\emsgine-core\src\namespaces\avr"
    )

    decoder_codegen.run(decoder_settings)

    # Path(r"D:\dev\world-embeddable-simulator\embdsim\src\decoder.rs").write_text(decoder_code)


if __name__ == "__main__":
    main()