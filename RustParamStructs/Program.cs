// Generetas param struct from ER param defs.
// https://github.com/soulsmods/Paramdex/ER/Defs
// https://github.com/vawser/Smithbox/tree/main/src/StudioCore/Assets/Paramdex/ER/Defs
// https://github.com/soulsmods/DSMapStudio/tree/master/src/StudioCore/Assets/Paramdex/ER/Defs
// etc..
using RustParamStructs;
using SoulsFormats;
using System.Security.Cryptography;
using System.Text;

namespace RustParamStruct
{
    class Program
    {
        static void Main(string[] args)
        {
#if DEBUG
            args = [
                ".\\ParamDex\\ER\\Defs",
                ".\\output"
            ];
#endif
            if (args.Length == 0)
            {
                Console.WriteLine("Error: No arguments provided.");
                Console.WriteLine("Please provide an input and output directory as an argument when running the application.");
                Console.WriteLine("Usage: RustParamStruct [input_directory] [output_directory]");
                return;
            }

            if (!Directory.Exists(args[0]))
            {
                Console.WriteLine("Error: Input Directory doens't exist.");
                return;
            }

            if (!Directory.Exists(args[1]))
            {
                Console.WriteLine($"Created output directory: {args[1]}");
                Directory.CreateDirectory(args[1]);
            }

            var files = Directory.GetFiles(args[0]);

            if (files.Length == 0)
            {
                Console.WriteLine("Warning: Directory is empty!");
                return;
            }

            var distDir = args[1];

            // Mod files
            var mod = new Mod();
            var defMod = new Mod();

            // Generate Trait
            mod.Append(ParamTrait.Generate(args[1]));

            mod.Append("defs");

            var def_mod_rs = new StringBuilder();
            var mod_rs = new StringBuilder("pub mod defs;\n");
            foreach (var file in files)
            {
                if (Path.GetExtension(file) != ".xml") continue;

                // Generate def struct
                defMod.Append(ParamDefStruct.Generate(file, distDir));

                // Generate struct
                mod.Append(ParamStruct.Generate(file, distDir));

            }

            // Generate Mod
            mod.Write(distDir);
            defMod.Write(Path.Join(distDir, "defs"));
        }
    }
}