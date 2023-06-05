const Login: React.FC = () => {
    return (
        <div className="auth-page">
            <form className="auth-form">
                <h1>Login</h1>
                <div className="input-box">
                    <input name="username" type="text" required={true} />
                    <span>Username</span>
                    <i/>
                </div>
                <div className="input-box">
                    <input name="password" type="password" required={true} />
                    <span>Password</span>
                    <i/>
                </div>
                <button type="submit">
                    Login
                </button>
            </form>
        </div>
    )
}

export default Login;
