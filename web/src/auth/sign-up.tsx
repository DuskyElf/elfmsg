import { useState, useRef } from "react";

type UserNameProp = {
    next: React.MouseEventHandler<HTMLButtonElement>,
    user_elem_complete: React.Dispatch<React.SetStateAction<boolean>> 
}

const UserNameInput: React.FC<UserNameProp> = ({next, user_elem_complete}) => {
    let username_elem = useRef<HTMLInputElement>(null!)
    let email_elem = useRef<HTMLInputElement>(null!)

    const key_down: React.KeyboardEventHandler = (event: React.KeyboardEvent<HTMLButtonElement>) => {
        user_elem_complete(username_elem.current.checkValidity() && email_elem.current.checkValidity())
    }
    return (<>
        <div className="input-box">
            <input name="username" onKeyDown={key_down} ref={username_elem} type="text" required={true} />
            <span>Username</span>
            <i/>
        </div>
        <div className="input-box">
            <input name="email" onKeyDown={key_down} ref={email_elem} type="text" required={true} />
            <span>Email</span>
            <i/>
        </div>
        <button onClick={next}>
            Next
        </button>
    </>)
}

const PasswordInput: React.FC = () => {
    return (<>
        <div className="input-box">
            <input name="password" type="password" required={true} />
            <span>Password</span>
            <i/>
        </div>
        <div className="input-box">
            <input name="re-password" type="password" required={true} />
            <span>Reenter Password</span>
            <i/>
        </div>
        <button type="submit">
            Sign Up
        </button>
    </>)
}

type Phase = "username" | "password"
const SignUp: React.FC = () => {
    let [phase, set_phase] = useState<Phase>("username");
    let [user_complete, set_user_complete] = useState(false);

    const next = (event: React.MouseEvent<HTMLButtonElement>) => {
        if (!user_complete)
            return;
        event.preventDefault()
        set_phase("password")
    }

    return (
        <div className="auth-page">
            <form className="auth-form">
                <h1>Sign Up</h1>
                {
                    phase == "username"?
                        <UserNameInput next={next} user_elem_complete={set_user_complete} />:
                        <PasswordInput />
                }
            </form>
        </div>
    )
}

export default SignUp;
