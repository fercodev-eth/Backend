const express = require('express');
const bodyParser = require('body-parser');

const app = express();
const port = 3000;

app.use(bodyParser.json());

// Placeholder for the database
const eventsDB = [];

app.post('/api/events', (req, res) => {
    const { name, nit, email, website, goal_amount, deadline } = req.body;

    // TODO: Call the smart contract to create the event on-chain

    const newEvent = {
        id: eventsDB.length,
        name,
        nit,
        email,
        website,
        goal_amount,
        deadline,
        amount_raised: 0,
        status: 'Open'
    };

    eventsDB.push(newEvent);

    res.status(201).json(newEvent);
});

app.get('/api/events', (req, res) => {
    // TODO: Fetch events from the smart contract and merge with DB data
    res.json(eventsDB);
});

app.get('/api/events/:id', (req, res) => {
    const eventId = parseInt(req.params.id, 10);
    // TODO: Fetch event from the smart contract and merge with DB data
    const event = eventsDB.find(e => e.id === eventId);

    if (event) {
        res.json(event);
    } else {
        res.status(404).send('Event not found');
    }
});

app.listen(port, () => {
    console.log(`Backend server listening at http://localhost:${port}`);
});