import React from 'react';


class Rant extends React.Component {
    render() {
        return ( 
            <table>
                <thead>
                  <tr>
                    <th>User</th>
                    <th>Rant</th>
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <td>Charlie</td>
                    <td>Life sucks and then you die</td>
                  </tr>
                  <tr>
                    <td>Mac</td>
                    <td>Existence is pain</td>
                  </tr>
                  <tr>
                    <td>Dee</td>
                    <td>Britney survived 2007, you can handle today.</td>
                  </tr>
                  <tr>
                    <td>Dennis</td>
                    <td>My hands are cold</td>
                  </tr>
                </tbody>
              </table>
            )
    }
}

export default Rant