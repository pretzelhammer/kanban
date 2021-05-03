const express = require('express');
const router = express.Router();
const db = require('./db');

// parse JSON bodies

router.use(express.json());

// auth middleware

router.use(async (req, res, next) => {
    let auth = req.get('Authorization');
    if (!auth) {
        res.sendStatus(400);
        return;
    }
    let [auth_type, token_id] = auth.split(' ');
    if (auth_type != 'Bearer') {
        res.sendStatus(400);
        return;
    }
    let validToken = await db.validateToken(token_id);
    if (validToken) {
        next();
    } else {
        res.sendStatus(401);
    }
});

// board routes

router.get('/boards', async (_req, res) => {
    let boards = await db.boards();
    res.json(boards);
});

router.get('/boards/:boardId/summary', async (req, res) => {
    let { boardId } = req.params;
    boardId = Number.parseInt(boardId);
    if (Number.isNaN(boardId)) {
        res.sendStatus(400);
        return;
    }
    let boardSummary = await db.boardSummary(boardId);
    res.json(boardSummary);
});

router.post('/boards', async (req, res) => {
    let newBoard = req.body;
    if (!newBoard || !newBoard.name) {
        res.sendStatus(400);
        return;
    }
    let createdBoard = await db.createBoard(newBoard);
    res.json(createdBoard);
});

router.delete('/boards/:boardId', async (req, res) => {
    let { boardId } = req.params;
    boardId = Number.parseInt(boardId);
    if (Number.isNaN(boardId)) {
        res.sendStatus(400);
        return;
    }
    await db.destroyBoard(boardId);
    res.end();
});

// card routes

router.get('/boards/:boardId/cards', async (req, res) => {
    let { boardId } = req.params;
    boardId = Number.parseInt(boardId);
    if (Number.isNaN(boardId)) {
        res.sendStatus(400);
        return;
    }
    let cards = await db.cards(boardId);
    res.json(cards);
});

router.post('/cards', async (req, res) => {
    let newCard = req.body;
    if (!newCard || !newCard.boardId || !newCard.description) {
        res.sendStatus(400);
        return;
    }
    let createdCard = await db.createCard(newCard);
    res.json(createdCard);
});

router.patch('/cards/:cardId', async (req, res) => {
    let { cardId } = req.params;
    cardId = Number.parseInt(cardId);
    if (Number.isNaN(cardId)) {
        res.sendStatus(400);
        return;
    }
    let updateCard = req.body;
    if (!updateCard || !updateCard.description || !updateCard.status) {
        res.sendStatus(400);
        return;
    }
    let updatedCard = await db.updateCard(cardId, updateCard);
    res.json(updatedCard);
});

router.delete('/cards/:cardId', async (req, res) => {
    let { cardId } = req.params;
    cardId = Number.parseInt(cardId);
    if (Number.isNaN(cardId)) {
        res.sendStatus(400);
        return;
    }
    await db.destroyCard(cardId);
    res.end();
});

module.exports = router;
