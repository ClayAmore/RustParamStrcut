using SoulsFormats;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RustParamStructs
{
    internal class ParamDefStruct
    {

        public static string Generate(string src, string distDir)
        {
            var sb = new StringBuilder();
            var paramdef = PARAMDEF.XmlDeserialize(src);
            strcut(sb, paramdef);
            implStruct(sb, paramdef);
            implDefault(sb, paramdef);
            File.WriteAllText(Path.Join(distDir, "defs", $"{paramdef.ParamType}.rs"), sb.ToString());
            return paramdef.ParamType;
        }

        // STRUCT
        private static void strcut(StringBuilder sb, PARAMDEF paramdef)
        {
            sb.AppendLine("#[repr(C, packed)]");
            sb.AppendLine("#[derive(Clone)]");
            sb.AppendLine("#[allow(unused,non_snake_case, non_camel_case_types)]");
            sb.AppendLine($"pub struct {paramdef.ParamType} {{");
            var bitsCounter = 0;
            for (var i = 0; i < paramdef.Fields.Count; i++)
            {
                // Field
                var field = paramdef.Fields[i];

                // Name
                var name = $"pub {(field.InternalName == "type" ? "r#type" : field.InternalName)}";

                // Bit type name
                if (Util.IsBit(field)) name = $"bits_{bitsCounter}";

                // Add name
                sb.AppendLine($"\t{name}: {Util.GetRustTypeFromDisplayType(field)},");

                // If bit loop until bit is done
                if (Util.IsBit(field))
                {
                    var j = 0;
                    while (true)
                    {
                        if (!Util.IsBit(field))
                        {
                            if (j < Util.GetValueSize(paramdef.Fields[i - 1].DisplayType))
                            {
                                bitsCounter++;
                            }
                            break;
                        }

                        // Check if bit is done 
                        j += field.BitSize;
                        if (j > Util.GetValueSize(field.DisplayType))
                        {
                            bitsCounter++;
                            j = 0;
                            break;
                        }

                        // Increment field
                        if (++i >= paramdef.Fields.Count) break;
                        field = paramdef.Fields[i];
                    }
                }
            }
            sb.AppendLine($"}}");
        }

        // IMPL STRUCT
        private static void implStruct(StringBuilder sb, PARAMDEF paramdef)
        {
            sb.AppendLine("#[allow(unused,non_snake_case, non_camel_case_types)]");
            sb.AppendLine($"impl {paramdef.ParamType} {{");
            var bitsCounter = 0;
            for (var i = 0; i < paramdef.Fields.Count; i++)
            {
                var field = paramdef.Fields[i];
                var j = 0;
                while (Util.IsBit(field))
                {
                    var name = field.InternalName;
                    if (name == "type") name = "r#type";
                    sb.AppendLine($"\tpub fn {name}(&self) -> bool {{\t");
                    sb.AppendLine($"\t\t\tself.bits_{bitsCounter} & (1 << {j}) != 0");
                    sb.AppendLine($"\t}}");
                    j += field.BitSize;
                    if (j > Util.GetValueSize(field.DisplayType))
                    {
                        bitsCounter++;
                        j = 0;
                    }
                    if (++i >= paramdef.Fields.Count) break;
                    field = paramdef.Fields[i];
                }
            }
            sb.AppendLine($"}}");
        }

        // IMPL DEFAULT
        private static void implDefault(StringBuilder sb, PARAMDEF paramdef)
        {
            sb.AppendLine($"impl Default for {paramdef.ParamType} {{");
            sb.AppendLine($"\tfn default() -> Self {{");
            sb.AppendLine($"\t\tSelf {{");
            var bitsCounter = 0;
            for (var i = 0; i < paramdef.Fields.Count; i++)
            {
                // Field
                var field = paramdef.Fields[i];

                if (Util.IsBit(field))
                {
                    // bits_x
                    var name = $"bits_{bitsCounter}";
                    sb.AppendLine($"\t\t\t {name}: {Util.GetDefaultValue(field)},");

                    var j = 0;
                    while (true)
                    {
                        if (!Util.IsBit(field))
                        {
                            if (j < Util.GetValueSize(paramdef.Fields[i - 1].DisplayType))
                            {
                                bitsCounter++;
                            }
                            break;
                        }

                        // Check if bit is done 
                        j += field.BitSize;
                        if (j > Util.GetValueSize(field.DisplayType))
                        {
                            bitsCounter++;
                            j = 0;
                            break;
                        }

                        // Increment field
                        if (++i >= paramdef.Fields.Count) break;
                        field = paramdef.Fields[i];
                    }
                }
                else
                {
                    var name = field.InternalName;
                    if (name == "type") name = "r#type";
                    sb.AppendLine($"\t\t\t {name}: {Util.GetDefaultValue(field)},");
                }
            }
            sb.AppendLine($"\t\t}}");
            sb.AppendLine($"\t}}");
            sb.AppendLine($"}}");
        }
    }
}
