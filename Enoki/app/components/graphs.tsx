import {JSX} from "react";
import { Bar, Line, Scatter, Bubble, Pie } from "react-chartjs-2";

// TODO: Define proper UI styling wrappers to match the rest of the app
export function LineGraph(data: any, width: number, height: number): JSX.Element {
    return(
        <Line data={data} width={width} height={height} options={{ maintainAspectRatio: false }} />
    )
}

export function BarGraph(data: any, width: number, height: number): JSX.Element {
    return(
        <Bar data={data} width={width} height={height} options={{ maintainAspectRatio: false }} />
    )
}

export function PieChart(data: any, width: number, height: number): JSX.Element {
    return(
        <Pie data={data} width={width} height={height} options={{ maintainAspectRatio: false }} />
    )
}

export function ScatterChart(data: any, width: number, height: number): JSX.Element {
    return(
        <Scatter data={data} width={width} height={height} options={{ maintainAspectRatio: false }} />
    )
}

export function BubbleChart(data: any, width: number, height: number): JSX.Element {
    return(
        <Bubble data={data} width={width} height={height} options={{ maintainAspectRatio: false }} />
    )
}