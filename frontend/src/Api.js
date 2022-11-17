import React from "react";

class App extends React.Component {
    state = {
        data: [],
    }

    componentDidMount() {
        const url =       
        //'https://en.wikipedia.org/w/api.php?action=opensearch&search=Seona+Dancing&format=json&origin=*';
        'http://localhost:8080/';

        fetch(url)
            .then((result) => result.json())
            
            .then((result) => {
                console.log("recibimos result " + result.toString());
                this.setState({
                    data: result,
                })
            })
    }

    render() {
        const { data } = this.state;
        console.log(data)
        const result = data.map((entry, index) => {
            return <li key={index}>{entry}</li>
          })
        return <ul>{result}</ul>
        //return <div>{this.state.data}</div>
    }
}

export default App