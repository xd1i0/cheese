import "./App.css";
import { useState, useRef, useEffect } from "react";

import wP from "./assets/pieces/wP.svg?url";
import wR from "./assets/pieces/wR.svg?url";
import wN from "./assets/pieces/wN.svg?url";
import wB from "./assets/pieces/wB.svg?url";
import wQ from "./assets/pieces/wQ.svg?url";
import wK from "./assets/pieces/wK.svg?url";

import bP from "./assets/pieces/bP.svg?url";
import bR from "./assets/pieces/bR.svg?url";
import bN from "./assets/pieces/bN.svg?url";
import bB from "./assets/pieces/bB.svg?url";
import bQ from "./assets/pieces/bQ.svg?url";
import bK from "./assets/pieces/bK.svg?url";

type Piece = {
  row: number;
  col: number;
  type: string;
  color: "w" | "b";
};

type Board = (Piece | null)[][];

type Move = {
  row: number;
  col: number;
};

type PieceKey =
  | "wP" | "wR" | "wN" | "wB" | "wQ" | "wK"
  | "bP" | "bR" | "bN" | "bB" | "bQ" | "bK";

const pieceImages: Record<PieceKey, string> = {
  wP, wR, wN, wB, wQ, wK,
  bP, bR, bN, bB, bQ, bK,
};

type MoveRow = {
  white: string | null;
  black: string | null;
};

interface EngineLine {
  score: number;
  moves: string;
}

const lines: EngineLine[] = [
  { score: +0.16, moves: "3. d4 exd4 4. Qxd4 Nc6 5. Qe3 b6 6. Nc3 Bc5 7. Qg5 Q..." },
  { score: -0.36, moves: "3. Ne2 d5 4. exd5 Qxd5 5. d4 Ne7 6. Nbc3 Qa5 7. dxe5 ..." },
  { score: -0.54, moves: "3. Bc4 Bc5 4. Nc3 Nc6 5. Nge2 d6 6. Nd5 Nge7 7. c3 N..." },
];

function App() {
  const [fen, setFen] = useState(
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
  );

  const [board, setBoard] = useState<Board>(() => FEN2Board(fen));
  const [selected, setSelected] = useState<{ row: number; col: number } | null>(null);
  const [moves, setMoves] = useState<Move[]>([]);
  const [moveHistory, setMoveHistory] = useState<MoveRow[]>([]);
  const [turn, setTurn] = useState<"w" | "b">("w");
  const [evaluation, setEvaluation] = useState<number>(-1.2);
  const [expanded, setExpanded] = useState<Set<number>>(new Set());
  const [analysisOn, setAnalysisOn] = useState(true);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const settingsRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (settingsRef.current && !settingsRef.current.contains(e.target as Node)) {
        setSettingsOpen(false);
      }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const toggle = (i: number) => {
    setExpanded(prev => {
      const next = new Set(prev);
      next.has(i) ? next.delete(i) : next.add(i);
      return next;
    });
  };

  function FEN2Board(fen: string): Board {
    const board: Board = Array.from({ length: 8 }, () => Array(8).fill(null));
    const position = fen.split(" ")[0];
    const rows = position.split("/");

    for (let r = 0; r < rows.length; r++) {
      let c = 0;
      for (const char of rows[r]) {
        if (char >= "1" && char <= "8") {
          c += Number(char);
        } else {
          board[r][c] = {
            row: r,
            col: c,
            type: char.toUpperCase(),
            color: char === char.toUpperCase() ? "w" : "b",
          };
          c++;
        }
      }
    }

    return board;
  }

  function scoreBadgeStyle(score: number): React.CSSProperties {
    const positive = score >= 0;
    return {
      background: positive ? "#F5F5F5" : "#312E2B",
      color: positive ? "#312E2B" : "#FFFFFF",
      fontSize: 13,
      fontWeight: 600,
      fontVariantNumeric: "tabular-nums",
      minWidth: 44,
      borderRadius: 4,
      padding: "3px 7px",
      textAlign: "center",
      flexShrink: 0,
    };
  }

  function formatScore(score: number): string {
    return (score >= 0 ? "+" : "") + score.toFixed(2);
  }

  function getMoves(row: number, col: number): Move[] {
    const piece = board[row][col];
    if (!piece) return [];

    if (piece.type === "N") {
      return [
        { row: row + 2, col: col + 1 },
        { row: row + 2, col: col - 1 },
        { row: row - 2, col: col + 1 },
        { row: row - 2, col: col - 1 },
        { row: row + 1, col: col + 2 },
        { row: row + 1, col: col - 2 },
        { row: row - 1, col: col + 2 },
        { row: row - 1, col: col - 2 },
      ].filter(m => m.row >= 0 && m.row < 8 && m.col >= 0 && m.col < 8 && (!board[m.row][m.col] || board[m.row][m.col]?.color !== piece.color));
    }

    if (piece.type === "P") {
      const direction = piece.color === "w" ? -1 : 1;
      const startRow = piece.color === "w" ? 6 : 1;
      const moves: Move[] = [];

      if (row + direction >= 0 && row + direction < 8 && !board[row + direction][col]) {
        moves.push({ row: row + direction, col });
        if (row === startRow && !board[row + 2 * direction][col]) {
          moves.push({ row: row + 2 * direction, col });
        }
      }
      if (col - 1 >= 0 && board[row + direction][col - 1] && board[row + direction][col - 1]?.color !== piece.color) {
        moves.push({ row: row + direction, col: col - 1 });
      }
      if (col + 1 < 8 && board[row + direction][col + 1] && board[row + direction][col + 1]?.color !== piece.color) {
        moves.push({ row: row + direction, col: col + 1 });
      }

      return moves;
    }

    if (piece.type === "K") {
      return [
        { row: row + 1, col },
        { row: row - 1, col },
        { row, col: col + 1 },
        { row, col: col - 1 },
        { row: row + 1, col: col + 1 },
        { row: row + 1, col: col - 1 },
        { row: row - 1, col: col + 1 },
        { row: row - 1, col: col - 1 },
      ].filter(m => m.row >= 0 && m.row < 8 && m.col >= 0 && m.col < 8 && (!board[m.row][m.col] || board[m.row][m.col]?.color !== piece.color));
    }

    if (piece.type === "R" || piece.type === "B" || piece.type === "Q") {
      const moves: Move[] = [];
      const directions: { dr: number; dc: number }[] = [];

      if (piece.type === "R" || piece.type === "Q") {
        directions.push({ dr: 1, dc: 0 }, { dr: -1, dc: 0 }, { dr: 0, dc: 1 }, { dr: 0, dc: -1 });
      }
      if (piece.type === "B" || piece.type === "Q") {
        directions.push({ dr: 1, dc: 1 }, { dr: 1, dc: -1 }, { dr: -1, dc: 1 }, { dr: -1, dc: -1 });
      }

      for (const { dr, dc } of directions) {
        let r = row + dr;
        let c = col + dc;

        while (r >= 0 && r < 8 && c >= 0 && c < 8) {
          const target = board[r][c];
          if (target === null) {
            moves.push({ row: r, col: c });
          } else {
            if (target.color !== piece.color) moves.push({ row: r, col: c });
            break;
          }
          r += dr;
          c += dc;
        }
      }

      return moves;
    }

    return [];
  }

  function toSquare(r: number, c: number) {
    return String.fromCharCode(97 + c) + (8 - r);
  }

  function handleSquareClick(row: number, col: number) {
    if (selected === null) {
      if (board[row][col]) {
        if (board[row][col]?.color !== turn) return;
        setSelected({ row, col });
        setMoves(getMoves(row, col));
      }
      return;
    }

    if (selected.row === row && selected.col === col) {
      setSelected(null);
      setMoves([]);
      return;
    }

    if (board[row][col]?.color === board[selected.row][selected.col]?.color) {
      setSelected({ row, col });
      setMoves(getMoves(row, col));
      return;
    }

    const piece = board[selected.row][selected.col];
    if (!piece) return;
    if (!moves.some(m => m.row === row && m.col === col)) return;
    if (piece.color !== turn) return;

    const newBoard = board.map(row => [...row]);
    newBoard[row][col] = piece;
    newBoard[selected.row][selected.col] = null;

    setBoard(newBoard);
    setSelected(null);

    const from = toSquare(selected.row, selected.col);
    const to = toSquare(row, col);
    let move = "";

    switch (piece.type) {
      case "P": move = board[row][col] ? `${from[0]}x${to}` : to; break;
      case "N": move = `N${board[row][col] ? "x" : ""}${to}`; break;
      case "B": move = `B${board[row][col] ? "x" : ""}${to}`; break;
      case "R": move = `R${board[row][col] ? "x" : ""}${to}`; break;
      case "Q": move = `Q${board[row][col] ? "x" : ""}${to}`; break;
      case "K": move = `K${board[row][col] ? "x" : ""}${to}`; break;
      default:  move = `${from}-${to}`;
    }

    setMoveHistory(prev => {
      const copy = [...prev];
      if (piece.color === "w") {
        copy.push({ white: move, black: null });
      } else {
        const last = copy[copy.length - 1];
        if (last && last.black === null) {
          copy[copy.length - 1] = { ...last, black: move };
        } else {
          copy.push({ white: null, black: move });
        }
      }
      return copy;
    });

    setMoves([]);
    setTurn(prev => (prev === "w" ? "b" : "w"));
  }

  function isHighlighted(r: number, c: number) {
    return moves.some(m => m.row === r && m.col === c);
  }

  function isCaptureMove(r: number, c: number) {
    const piece = board[selected?.row ?? -1]?.[selected?.col ?? -1];
    if (!piece) return false;
    return moves.some(m => m.row === r && m.col === c && board[r][c] !== null && board[r][c]?.color !== piece.color);
  }

  return (
    <main className="container">
      <div className="game-layout">

        {/* Eval bar — hidden when analysis is off */}
        {analysisOn && (
          <div className="evaluation-bar">
            <span
              className="eval-text"
              style={{
                top: evaluation < 0 ? "6px" : "auto",
                bottom: evaluation >= 0 ? "6px" : "auto",
                color: evaluation < 0 ? "#e8e3dc" : "#403D39",
              }}
            >
              {evaluation > 0 ? `${evaluation}` : `${Math.abs(evaluation)}`}
            </span>
          </div>
        )}

        {/* Board */}
        <div className="board-wrapper">
          <div className="chessboard">
            {Array.from({ length: 8 }).map((_, r) =>
              Array.from({ length: 8 }).map((_, c) => {
                const piece = board[r][c];
                return (
                  <div
                    key={`${r}-${c}`}
                    className={`square ${(r + c) % 2 === 0 ? "dark" : "light"}`}
                    onClick={() => handleSquareClick(r, c)}
                  >
                    <div
                      className={`square-overlay ${
                        selected?.row === r && selected?.col === c ? "selected" : ""
                      } ${isHighlighted(r, c) ? "possible-move" : ""} ${
                        isCaptureMove(r, c) ? "capture" : ""
                      }`}
                    />
                    {piece && (
                      <img
                        src={pieceImages[(piece.color + piece.type) as PieceKey]}
                        alt=""
                        className={`piece-img ${piece.color}`}
                      />
                    )}
                  </div>
                );
              })
            )}
          </div>
        </div>

        {/* Side panel */}
        <div className="side-panel-wrapper">

          {/* Top bar with settings */}
          <div className="side-panel-topbar">
            <div className="topbar-left">
              <button
                className={`toggle small ${analysisOn ? "on" : ""}`}
                onClick={() => setAnalysisOn(o => !o)}
                aria-label="Toggle analysis"
              >
                <span className="toggle-thumb" />
              </button>
              <span className="settings-label">Analysis</span>
            </div>

            <div className="settings-container" ref={settingsRef}>
              <button
                className="settings-btn"
                onClick={() => setSettingsOpen(o => !o)}
                aria-label="Settings"
              >
                ⚙
              </button>
              {settingsOpen && (
                <div className="settings-dropdown">
                  <span className="settings-empty">No settings yet</span>
                </div>
              )}
            </div>
          </div>

          {/* Engine lines — hidden when analysis is off */}
          {analysisOn && (
            <div className="engineMovePanel">
              {lines.map((line, i) => {
                const isExpanded = expanded.has(i);
                return (
                  <div key={i} className="engineMovePanel-row">
                    <span style={scoreBadgeStyle(line.score)}>
                      {formatScore(line.score)}
                    </span>
                    <span className={`engineMovePanel-moves ${isExpanded ? "expanded" : ""}`}>
                      {line.moves}
                    </span>
                    <button
                      className={`engineMovePanel-chevron${isExpanded ? " open" : ""}`}
                      onClick={() => toggle(i)}
                      aria-label={isExpanded ? "collapse line" : "expand line"}
                    >
                      ▾
                    </button>
                  </div>
                );
              })}
            </div>
          )}

          {/* Move history */}
          <div className="move-history">
            <div className="move-list">
              {moveHistory.length === 0 ? (
                <p className="empty-history">No moves yet</p>
              ) : (
                moveHistory.map((row, i) => (
                  <div key={i} className="move-row">
                    <span className="move-index">{i + 1}.</span>
                    <span className="white-move">{row.white ?? ""}</span>
                    <span className="black-move">{row.black ?? ""}</span>
                  </div>
                ))
              )}
            </div>
          </div>

          {/* Controls */}
          <div className="controls-pane">
            <span className="controls" onClick={() => {}}>⏮</span>
            <span className="controls" onClick={() => {}} style={{ fontSize: "12px" }}>◀</span>
            <span className="controls" onClick={() => {}} style={{ fontSize: "12px" }}>▶</span>
            <span className="controls" onClick={() => {}}>⏭</span>
          </div>

        </div>
      </div>
    </main>
  );
}

export default App;