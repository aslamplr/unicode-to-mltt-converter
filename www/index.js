import * as mlttConverter from 'unicode-to-mltt-converter';
import * as React from 'react';
import * as ReactDOM from 'react-dom';

import { Box, Button, Heading, Grommet, TextArea, Paragraph } from 'grommet';
import { Sync, Copy } from 'grommet-icons';

const theme = {
    global: {
        colors: {
            brand: '#228BE6',
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
    
    React.useEffect(() => {
        fetch("public/karthika.map").then((resp) => resp.text()).then((defaultCharmapVal) => {
            setCharMapValue(defaultCharmapVal);
        });
    }, []);

    const resultTextAreaEl = React.useRef(null);

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
                        gap='medium'
                    >
                        <Paragraph fill>
                        Malayalam Unicode to ML-TT Converter is an utility for converting Malayalam Unicode characters to 
                        corresponding ML-TT encoding. It uses default Karthika font character mapping. 
                        </Paragraph>
                        <TextArea
                            rows="10"
                            size='xsmall'
                            placeholder='loading default char map...'
                            value={charmapValue}
                            onChange={event => setCharMapValue(event.target.value)}
                        />
                        <TextArea
                            rows="10"
                            placeholder='type/paste unicode input here!'
                            value={inputValue}
                            onChange={event => setInputValue(event.target.value)}
                        />
                        <TextArea
                            rows="10"
                            ref={resultTextAreaEl}
                            placeholder='click Convert to see the results here!'
                            value={value}
                            onChange={event => setValue(event.target.value)}
                        />
                        <Button primary label='Convert' icon={<Sync />} onClick={convert} />
                        <Button label='Copy' icon={<Copy />} onClick={copyToClipboard} />
                    </Box>
                </Box>
            </Box>
        </Grommet>
    );
}


ReactDOM.render(<App />, document.getElementById('root'));

