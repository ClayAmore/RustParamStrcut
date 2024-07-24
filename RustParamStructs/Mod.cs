using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RustParamStructs
{
    internal class Mod
    {
        private StringBuilder _sb;

        public Mod() { 
            _sb = new StringBuilder();
        }

        public void Append(string name)
        {
            _sb.AppendLine("#[allow(unused,non_snake_case, non_camel_case_types)]");
            _sb.AppendLine($"pub mod {name};");
        }

        public void Write(string distDir)
        {
            File.WriteAllText(Path.Join(distDir, $"mod.rs"), _sb.ToString());
        }
    }
}
