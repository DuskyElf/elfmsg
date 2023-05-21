import ReactDOM from 'react-dom/client';
import * as rr from 'react-router-dom';

import AuthRouter from './auth/auth';

const router = rr.createBrowserRouter(
    rr.createRoutesFromElements(
        <rr.Route>
            <rr.Route index element={<h1>Hello World</h1>} />
            <rr.Route path="auth">
                <rr.Route path="login" element={<AuthRouter subPath="login" />} />
                <rr.Route path="sign-up" element={<AuthRouter subPath="sign-up" />} />
            </rr.Route>
        </rr.Route>
    )
);

const root = ReactDOM.createRoot(
    document.getElementById('root') as HTMLElement
);
root.render(
    <rr.RouterProvider router={router} />
);

