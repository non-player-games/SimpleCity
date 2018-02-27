import { Component } from './interfaces';
import { Home } from './components/home';

export interface RouteValue {
    component: Component;
    scope: string;
}
export interface Routes {
    readonly [index: string]: RouteValue;
}

export const routes: Routes = {
    '/': { component: Home, scope: 'home' }
};

export const initialRoute = '/';
