import * as mlttConverter from 'unicode-to-mltt-converter';
import * as React from 'react';
import * as ReactDOM from 'react-dom';

import { Box, Button, Heading, Grommet, TextArea } from 'grommet';
import { Notification } from 'grommet-icons';

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
    const convert = () => {
        const text = mlttConverter.convert(inputValue, charmapValue);
        setValue(text);
    }

    return (
        <Grommet theme={theme} full>
            <Box fill>
                <AppBar>
                    <Heading level='3' margin='none'>Unicode to ML-TT Converter</Heading>
                </AppBar>
                <Box direction='row' flex overflow={{ horizontal: 'hidden' }}>
                    <Box
                        flex
                        align='center'
                        justify='center'
                        pad={{ left: 'medium', right: 'small', vertical: 'small' }}
                        gap='medium'
                    >
                        <TextArea
                            size='xlarge'
                            placeholder='place char map content here'
                            value={charmapValue}
                            onChange={event => setCharMapValue(event.target.value)}
                        />
                        <TextArea
                            size='xlarge'
                            placeholder='type here'
                            value={inputValue}
                            onChange={event => setInputValue(event.target.value)}
                        />
                        <TextArea
                            size='xlarge'
                            placeholder='result goes here'
                            value={value}
                            onChange={event => setValue(event.target.value)}
                        />
                        <Button primary label='convert' icon={<Notification />} onClick={convert} />
                    </Box>
                </Box>
            </Box>
        </Grommet>
    );
}


ReactDOM.render(<App />, document.getElementById('root'));

