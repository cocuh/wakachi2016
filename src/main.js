var React = require('react');
var ReactDOM = require('react-dom');
var Tooltip = require('rc-tooltip');
import 'rc-tooltip/assets/bootstrap_white.css';

import 'whatwg-fetch';

const default_string = "私はお酒が趣味である。\
おさけを呑みながら過ごす時間が好きなのだ、まんがタイムきらら枠をみながらもすばらしきことだし、\
CSI科学捜査班を見ながらもつ煮を食べるのもまたよい。クラフトビールとウォッカが推し。\
あぁギネスビールがのみたい。(明日月曜)";


class Word extends React.Component {
    constructor() {
        super();
        this.state = {
            visible: false
        };
        this.onVisibleChange = this.onVisibleChange.bind(this);
    }

    onVisibleChange(visible) {
        this.setState({visible: visible});
    }

    render() {
        var word = this.props.word;
        var color = {
            "名詞": 'orange',
            "形容詞": 'blue',
            "動詞": 'green',
            "感動詞": 'pink'
        }[word.hinshi];

        var button;
        if (color) {
            button = (
                <button className={"ui button "+color}>
                    {word.tango}
                </button>
            );
        } else {
            button = (
                <span>
                    {word.tango}
                </span>
            );
        }

        const base_google_url = "https://www.google.co.jp/search?q=";
        if (word.kihonkei=="*"){
            var url = base_google_url + word.tango;
        }else{
            var url = base_google_url + word.kihonkei;
        }
        
        return (
            <Tooltip
                visible={this.state.visible}
                animation="zoom"
                onVisibleChange={this.onVisibleChange}
                trigger={["click", "hover"]}
                overlay={<span>{word.yomi} - {word.hinshi}<br/>{word.kihonkei}<br/><a  target="_blank" href={url}>ggr</a></span>}
                placement="top"
            >
                {button}
            </Tooltip>
        )
    }
}


class Form extends React.Component {
    constructor() {
        super();
        this.state = {value: default_string};
        this.handleSubmit = this.handleSubmit.bind(this);
        this.handleChange = this.handleChange.bind(this);
    }

    handleChange(event) {
        this.setState({value: event.target.value});
    }

    render() {
        return (
            <div className="column">
                <div className="ui form">
                    <h4>Input Text</h4>
                    <div className="field">
                        <textarea maxLength="256" onChange={this.handleChange} defaultValue={this.state.value}/>
                    </div>
                    <div className="ui submit button" onClick={this.handleSubmit}>Submit</div>
                </div>
            </div>
        );
    }

    handleSubmit(e) {
        this.props.onSubmit(this.state.value)
    }
}

class Result extends React.Component {
    render() {
        var words = this.props.words.map(function (word) {
            return (
                <Word key={word.id} word={word}/>
            );
        });
        return (
            <div className="column">
                <h4>Result</h4>
                <div style={{"margin": "5%", "lineHeight": "5em"}}>
                    <div className="ui massive">
                        {words}
                    </div>
                </div>
            </div>
        );
    }
}


class App extends React.Component {
    constructor() {
        super();
        this.state = {
            value: default_string, words: [
                {
                    hinshi: "名詞",
                    yomi: "ようじょ",
                    kihonkei: "ようじょりあん",
                    tango: "幼女",
                    id: 1
                },
                {
                    hinshi: "女子",
                    yomi: "は",
                    kihonkei: "は",
                    tango: "は",
                    id: 2
                }
            ]
        };
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    render() {
        return (
            <div className="ui one column stackable grid">
                <Form onSubmit={this.handleSubmit}/>
                <Result words={this.state.words}/>
            </div>
        );
    }

    handleSubmit(body) {
        var self = this;
        console.log("send");
        console.log(body);
        fetch('/query', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({body: body})
        })
            .then(function (res) {
                return res.json()
            })
            .then(function (res) {
                console.log(res);
                var is_success = res.is_success;
                if (is_success) {
                    self.setState({words: res.data});
                } else {
                    alert(res.msg);
                }
            });
    }
}

ReactDOM.render(
    <App />,
    document.getElementById('content')
);


