import * as mlttConverter from 'unicode-to-mltt-converter';
import * as React from 'react';
import * as ReactDOM from 'react-dom';
import * as googleTransliterate from "google-input-tool";

import { Box, Button, Heading, Grommet, TextArea, Paragraph, CheckBox } from 'grommet';
import { Sync, Copy } from 'grommet-icons';

const theme = {
    global: {
        colors: {
            brand: '#5970af',
        },
        font: {
            family: 'Roboto',
            size: '18px',
            height: '20px',
        }
    },
};

const AppBar = props => (
    <Box
        tag='header'
        direction='row'
        align='center'
        justify='between'
        background='brand'
        pad={{ left: 'medium', right: 'small', vertical: 'small' }}
        elevation='medium'
        style={{ zIndex: '1' }}
        {...props}
    />
);

const Input = _ => {
    const [value, setValue] = React.useState('');
    return (
        <TextArea
            placeholder='type here'
            value={value}
            onChange={event => setValue(event.target.value)}
        />
    );
}

const App = _ => {
    const [charmapValue, setCharMapValue] = React.useState('');
    const [inputValue, setInputValue] = React.useState('');
    const [value, setValue] = React.useState('');
    const [doTransliterate, setDoTransliterate] = React.useState(false);
    const [showCharMap, setShowCharMap] = React.useState(false);

    React.useEffect(() => {
        fetch("public/karthika.map").then((resp) => resp.text()).then((defaultCharmapVal) => {
            setCharMapValue(defaultCharmapVal);
        });
    }, []);

    const resultTextAreaEl = React.useRef(null);

    const transliterateAndSetInputValue = (text) => {
        if (doTransliterate) {
            const lastChar = text && text.substring(text.length - 1);
            if (lastChar && lastChar === " " && text.length > inputValue.length) {
                const words = text.split(" ");
                const lastWord = words[words.length - 2];
                const request = new XMLHttpRequest();
                const inputLanguage = "ml-t-i0-und"; // malayalam
                googleTransliterate(request, lastWord, inputLanguage, 8).then((transliteration) => {
                    const [first_one] = transliteration;
                    const newText = [...words.slice(0, words.length - 2), first_one, ""].join(" ");
                    setInputValue(newText);
                }).catch((err) => {
                    console.error("[🛑 transliteration] ➡️ ", err);
                });
            }
        }
        setInputValue(text);
    }

    const convert = () => {
        const text = mlttConverter.convert(inputValue, charmapValue);
        setValue(text);
    }

    const copyToClipboard = (event) => {
        resultTextAreaEl.current.select();
        document.execCommand('copy');
        event.target.focus();
    }

    return (
        <Grommet theme={theme} full>
            <Box fill>
                <AppBar>
                    <Heading level='3' margin='none'>Malayalam unicode to ML-TT Converter</Heading>
                </AppBar>
                <Box direction='row' flex overflow={{ horizontal: 'hidden' }}>
                    <Box
                        flex
                        align='center'
                        justify='center'
                        pad={{ left: 'medium', right: 'small', vertical: 'small' }}
                        gap='small'
                    >
                        <Paragraph fill="horizontal">
                            Malayalam Unicode to ML-TT Converter is an utility for converting Malayalam Unicode characters to
                            corresponding ML-TT encoding. It uses default Karthika font character mapping.
                        </Paragraph>
                        <Box fill="horizontal" direction="row" gap="medium">
                            <CheckBox
                                checked={doTransliterate}
                                label="Enable transliterate"
                                onChange={event => setDoTransliterate(event.target.checked)}
                            />
                            <CheckBox
                                checked={showCharMap}
                                label="Show char map"
                                onChange={event => setShowCharMap(event.target.checked)}
                            />
                        </Box>
                        {showCharMap && <TextArea
                            rows="10"
                            size='xsmall'
                            placeholder='loading default char map...'
                            value={charmapValue}
                            onChange={event => setCharMapValue(event.target.value)}
                        />}
                        <TextArea
                            rows="10"
                            placeholder='type/paste unicode input here!'
                            value={inputValue}
                            onChange={event => transliterateAndSetInputValue(event.target.value)}
                        />
                        <TextArea
                            style={value ? { fontFamily: "ml-ttkarthikanormal" } : {}}
                            rows="10"
                            ref={resultTextAreaEl}
                            placeholder='click Convert to see the results here!'
                            value={value}
                            onChange={event => setValue(event.target.value)}
                        />
                        <Box gap="small" direction="row" height="60px">
                            <Button primary label='Convert' icon={<Sync />} onClick={convert} />
                            <Button label='Copy' icon={<Copy />} onClick={copyToClipboard} />
                        </Box>
                    </Box>
                </Box>
            </Box>
        </Grommet>
    );
}


ReactDOM.render(<App />, document.getElementById('root'));

