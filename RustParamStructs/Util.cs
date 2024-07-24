using SoulsFormats;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace RustParamStructs
{
    public class Util
    {
        public static int GetValueSize(PARAMDEF.DefType type)
        {
            switch (type)
            {
                case PARAMDEF.DefType.s8: return 7;
                case PARAMDEF.DefType.u8: return 7;
                case PARAMDEF.DefType.s16: return 15;
                case PARAMDEF.DefType.u16: return 15;
                case PARAMDEF.DefType.s32: return 32;
                case PARAMDEF.DefType.u32: return 31;
                case PARAMDEF.DefType.b32: return 31;
                case PARAMDEF.DefType.f32: return 31;
                case PARAMDEF.DefType.angle32: return 31;
                case PARAMDEF.DefType.f64: return 63;
                case PARAMDEF.DefType.dummy8: return 7;
                case PARAMDEF.DefType.fixstr: return 7;
                case PARAMDEF.DefType.fixstrW: return 7;

                default:
                    throw new NotImplementedException($"No value size specified for {nameof(PARAMDEF.DefType)}.{type}");
            }
        }

        public static bool IsBitType(PARAMDEF.DefType type)
        {
            switch (type)
            {
                case PARAMDEF.DefType.u8:
                case PARAMDEF.DefType.u16:
                case PARAMDEF.DefType.u32:
                case PARAMDEF.DefType.dummy8:
                    return true;

                default:
                    return false;
            }
        }

        public static bool IsBit(PARAMDEF.Field field)
        {
            return IsBitType(field.DisplayType) && field.BitSize != -1;
        }

        public static string GetRustTypeFromDisplayType(PARAMDEF.Field field)
        {
            switch (field.DisplayType)
            {
                case PARAMDEF.DefType.s8: { return "i8"; }
                case PARAMDEF.DefType.u8: { return $"u8"; }
                case PARAMDEF.DefType.s16: { return "i16"; }
                case PARAMDEF.DefType.u16: { return $"i16"; }
                case PARAMDEF.DefType.s32: { return "i32"; }
                case PARAMDEF.DefType.u32: { return $"i32"; }
                case PARAMDEF.DefType.b32: { return "u32"; }
                case PARAMDEF.DefType.f32: { return "f32"; }
                case PARAMDEF.DefType.angle32: { return "f32"; }
                case PARAMDEF.DefType.f64: { return "f64"; }
                case PARAMDEF.DefType.dummy8:
                    {
                        if (field.BitSize != -1)
                        {
                            return $"u8";
                        }
                        return $"[u8;{field.ArrayLength}]";
                    }
                case PARAMDEF.DefType.fixstr: { return $"[u8;{field.ArrayLength}]"; }
                case PARAMDEF.DefType.fixstrW: { return $"[u8;{field.ArrayLength}]"; }
                default: { throw new InvalidDataException("Displaytype not recoginzed!"); }
            }
        }


        public static string GetDefaultValue(PARAMDEF.Field field)
        {
            switch (field.DisplayType)
            {
                case PARAMDEF.DefType.s8: { return $"{field.Default}"; }
                case PARAMDEF.DefType.u8: { return $"{field.Default}"; }
                case PARAMDEF.DefType.s16: { return $"{field.Default}"; }
                case PARAMDEF.DefType.u16: { return $"{field.Default}"; }
                case PARAMDEF.DefType.s32: { return $"{field.Default}"; }
                case PARAMDEF.DefType.u32: { return $"{field.Default}"; }
                case PARAMDEF.DefType.b32: { return $"{field.Default}"; }
                case PARAMDEF.DefType.f32:
                    {
                        var output = $"{field.Default}".Replace(",", ".");
                        if (!output.Contains("."))
                        {
                            output += ".";
                        }
                        return output;
                    }
                case PARAMDEF.DefType.angle32:
                    {
                        var output = $"{field.Default}".Replace(",", ".");
                        if (!output.Contains("."))
                        {
                            output += ".";
                        }
                        return output;
                    }
                case PARAMDEF.DefType.f64:
                    {
                        var output = $"{field.Default}".Replace(",", ".");
                        if (!output.Contains("."))
                        {
                            output += ".";
                        }
                        return output;
                    }
                case PARAMDEF.DefType.dummy8:
                    {
                        if (field.BitSize != -1)
                        {
                            return $"0";
                        }
                        return $"[0;{field.ArrayLength}]";
                    }
                case PARAMDEF.DefType.fixstr: { return $"[0;{field.ArrayLength}]"; }
                case PARAMDEF.DefType.fixstrW: { return $"[0;{field.ArrayLength}]"; }
                default: { throw new InvalidDataException("Displaytype not recoginzed!"); }
            }
        }
    }
}
