// http://vitaly-t.github.io/pg-promise/module-pg-promise.html
const pgp = require('pg-promise')({
    pgNative: true,
    noWarnings: true,
});
// https://github.com/vitaly-t/pg-promise/wiki/Connection-Syntax#configuration-object
const pool = pgp({
    connectionString: process.env.DATABASE_URL,
    // no min connection configuration setting
    max: 100, // pool max connections in single-threaded mode
});

const db = {
    pool,
    async boards() {
        let boards = await this.pool.any('select * from boards');
        // console.log(boards);
        return boards;
    },
    async boardSummary(boardId) {
        let counts = await this.pool.any('select count(*), status from cards where cards.board_id = $1 group by status', [boardId]);
        // console.log(counts);
        let summary = {
            todo: 0,
            doing: 0,
            done: 0,
        };
        for (let count of counts) {
            summary[count.status] += parseInt(count.count, 10);
        }
        return summary;
    },
    async createBoard(newBoard) {
        let createdBoard = await this.pool.one('insert into boards (name) values ($1) returning *', [newBoard.name]);
        // console.log(createdBoard);
        return createdBoard;
    },
    async destroyBoard(boardId) {
        let result = await this.pool.none('delete from boards where id = $1', [boardId]);
        // console.log(result);
        return result;
    },
    async cards(boardId) {
        let cards = await this.pool.any('select * from cards where board_id = $1', [boardId]);
        // console.log(cards);
        return cards;
    },
    async createCard(newCard) {
        let createdCard = await this.pool.one('insert into cards (board_id, description) values ($1, $2) returning *', [newCard.boardId, newCard.description]);
        // console.log(createdCard);
        return createdCard;
    },
    async updateCard(cardId, cardUpdate) {
        let updatedCard = await this.pool.one('update cards set description = $1, status = $2 where id = $3 returning *', [cardUpdate.description, cardUpdate.status, cardId]);
        // console.log(updatedCard);
        return updatedCard;
    },
    async destroyCard(cardId) {
        let result = await this.pool.none('delete from cards where id = $1', [cardId]);
        // console.log(result);
        return result;
    },
    async validateToken(token_id) {
        let result = await this.pool.any('select * from tokens where id = $1 and expired_at > current_timestamp', [token_id]);
        return !!result.length;
    },
    end() {
        this.pool.$pool.end();
    },
};

module.exports = db;
