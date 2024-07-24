using SoulsFormats;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.Intrinsics.X86;
using System.Text;
using System.Threading.Tasks;

namespace RustParamStructs
{
    internal class ParamStruct
    {
        public static string Generate(string src, string distDir)
        {
            var sb = new StringBuilder();
            var paramdef = PARAMDEF.XmlDeserialize(src);

            var param = Path.GetFileNameWithoutExtension(src);

            sb.AppendLine("use super::{");
            sb.AppendLine($"\tdefs::{paramdef.ParamType}::{paramdef.ParamType},");
            sb.AppendLine("\tparam_trait::Param");
            sb.AppendLine("};");

            sb.AppendLine($"pub struct {param} {{");
            sb.AppendLine($"\tpub data: &'static {paramdef.ParamType}");
            sb.AppendLine("}\n");

            sb.AppendLine($"impl Param for {param} {{");
            sb.AppendLine($"\ttype ParamType = {paramdef.ParamType};");
            sb.AppendLine($"\t\tconst PARAM_NAME: &'static str = \"{param}\";");
            sb.AppendLine("}");

            File.WriteAllText(Path.Join(distDir, $"{param}.rs"), sb.ToString());
            return param;
        }
    }
}
