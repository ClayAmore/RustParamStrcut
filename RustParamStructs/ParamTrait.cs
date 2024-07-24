using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RustParamStructs
{
    internal class ParamTrait
    {
        public static string Generate(string distDir)
        {
            var sb = new StringBuilder();
            sb.AppendLine("pub trait Param {");
            sb.AppendLine("\ttype ParamType;");
            sb.AppendLine("\tconst PARAM_NAME: &'static str;");
            sb.AppendLine("}");

            var name = "param_trait";
            File.WriteAllText(Path.Join(distDir, $"{name}.rs"), sb.ToString());
            return name;
        }
    }
}
