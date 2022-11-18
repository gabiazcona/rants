import React from "react";

class App extends React.Component {
    state = {
        rants: [],
    }

    componentDidMount() {
        const url =       
        'http://localhost:8080/';

        fetch(url)
            .then((result) => result.json())
            
            .then((result) => {
                this.setState({
                    rants: result,
                })
            })
    }

    render() {
        const { rants } = this.state;
        console.log(rants)
        const result = rants.map((rant, index) => {
            return (<div key={index}>
                <h2>{rant.title}</h2><p>from {rant.username}</p>
                <h2>{rant.body}</h2><br/>
            </div>)
          })
        return <div>{result}</div>
    }
}

export default App