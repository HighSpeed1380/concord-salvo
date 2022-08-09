import { UseFormMethods } from "react-hook-form";

import { Text, Localizer } from "preact-i18n";

import { Category, InputBox } from "@revoltchat/ui";

import { I18nError } from "../../context/Locale";

type FieldType =
    | "email"
    | "username"
    | "password"
    | "invite"
    | "reg_email"
    | "date"
    | "current_password";

type Props = Omit<JSX.HTMLAttributes<HTMLInputElement>, "children" | "as"> & {
    type: FieldType;
    showOverline?: boolean;
    register: UseFormMethods["register"];
    error?: string;
    name?: string;
};

export default function FormField({
    type,
    register,
    showOverline,
    error,
    name,
    ...props
}: Props) {
    return (
        <>
            {showOverline && (
                <Category compact>
                    <I18nError error={error}>
                        <Text id={`login.${type}`} />
                    </I18nError>
                </Category>
            )}
            <Localizer>
                <InputBox
                    placeholder={
                        (
                            <Text id={`login.enter.${type}`} />
                        ) as unknown as string
                    }
                    name={
                        type === "current_password" ? "password" : name ?? type
                    }
                    type={
                        type === "invite" || type === "username"
                            ? "text"
                            : type === "current_password"
                            ? "password"
                            : type
                    }
                    // See https://github.com/mozilla/contain-facebook/issues/783
                    className="fbc-has-badge"
                    ref={register(
                        type === "password" || type === "current_password"
                            ? {
                                  validate: (value: string) =>
                                      value.length === 0
                                          ? "Required Field"
                                          : value.length < 8
                                          ? "Too Short"
                                          : value.length > 1024
                                          ? "Too Long"
                                          : undefined,
                              }
                            : type === "email" || type === "reg_email"
                            ? {
                                  required: "Required Field",
                                  pattern: {
                                      value: /^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$/i,
                                      message: "Invalid Email",
                                  },
                              }
                            : type === "username"
                            ? {
                                  validate: (value: string) =>
                                      value.length === 0
                                          ? "Required Field"
                                          : value.length < 2
                                          ? "Too Short"
                                          : value.length > 32
                                          ? "Too Long"
                                          : undefined,
                              }
                            : { required: "Required Field" },
                    )}
                    {...props}
                />
            </Localizer>
        </>
    );
}
