import Login from './login'
import SignUp from './sign-up'

type authRouterProc = {
    subPath: "login" | "sign-up"
}

const AuthRouter: React.FC<authRouterProc> = ({subPath}) => {
    switch (subPath) {
        case "login":
            return <Login />
        case "sign-up":
            return <SignUp />
    }
}

export default AuthRouter;
