/*
 *       反復深化 (何ステップ残して各状態に至ったかを記録する)
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_NODES 8                     /* 大蛇の節の最大数 */
#define MAX_ROCKS 100                   /* 岩の最大数 */
#define MAX_STEPS 20                    /* 最長ステップ数 */
#define RADIUS ((MAX_STEPS+2)/2+MAX_NODES+1)    /* 考慮に値する領域の半径 */

#define SIZE            (RADIUS*2+1)    /* 領域の幅や高さ */

char pits[SIZE*SIZE];                   /* 各窪みの状態 */

#define ROCK            (-2)
#define EMPTY           (-1)
#define SNAKE(n)        (n)
#define IS_SNAKE(x)     (0 <= (x) && (x) < nNodes)
#define SNAKE_MARK(n)   ((n) + '1')

#define P_INDEX(x, y)   (((x)+RADIUS)*SIZE+(y)+RADIUS)
#define PIT(x, y)               (pits[P_INDEX(x, y)])

#define N_DIRS          6               /* 方位の数 */
#define N_JUNC          3               /* 節間の接続の向きの数 */

int dX[N_DIRS] = { 0, 1, 1, 0, -1, -1 };        /* 各方位に対する dx */
int dY[N_DIRS] = { -1, -1, 0, 1, 1, 0 };        /* 各方位に対する dy */

int posToDir[3][3] = {
        { -1, 5, 4 },
        { 0, -1, 3 },
        { 1, 2, -1 }
};

#define DIR(dx, dy) (posToDir[(dx)+1][(dy)+1])  /* dx, dy を方位へ変換する */

int nNodes;                                     /* 大蛇の節の数 */
int nRocks;                                     /* 岩の数 */
int nodeX[MAX_NODES], nodeY[MAX_NODES];         /* 大蛇の節の位置 */

/* 何ステップ残してその状態に到達したかを記録する */
#define D_BITS  5                               /* MAX_STEPS < 2^D_BITS */

int visitRest[(SIZE*SIZE*N_DIRS*N_JUNC*N_JUNC*N_JUNC*N_JUNC*N_JUNC*N_JUNC
                           + sizeof(int)*8/D_BITS - 1) / (sizeof(int)*8/D_BITS)];

int readData ()
{
        int rockX[MAX_ROCKS], rockY[MAX_ROCKS];
        int goalX, goalY;
        int i, x, y;

        /* データを読み込む */
        if (scanf("%d", &nNodes) == 0 || nNodes <= 0)
                return 0;

        for (i = 0; i < nNodes; i++) {
                if (scanf("%d%d", &nodeX[i], &nodeY[i]) != 2)
                        return 0;
        }

        if (scanf("%d", &nRocks) == 0)
                return 0;

        for (i = 0; i < nRocks; i++) {
                if (scanf("%d%d", &rockX[i], &rockY[i]) != 2)
                        return 0;
        }

        if (scanf("%d%d", &goalX, &goalY) != 2)
                return 0;

        /* pits を初期化する */
        for (x = -RADIUS; x <= RADIUS; x++) {
                for (y = -RADIUS; y <= RADIUS; y++)
                        PIT(x, y) = EMPTY;
        }

        /* 科学者の位置を (0,0) として、蛇や岩の位置を pits へコピー */
        for (i = 0; i < nNodes; i++) {
                nodeX[i] -= goalX;
                nodeY[i] -= goalY;
                PIT(nodeX[i], nodeY[i]) = SNAKE(i);
        }

        for (i = 0; i < nRocks; i++) {
                x = rockX[i] - goalX;
                y = rockY[i] - goalY;
                if (-RADIUS <= x && x <= RADIUS && -RADIUS <= y && y <= RADIUS)
                        PIT(x, y) = ROCK;
        }

        return 1;
}

/*
 *      現在の状態を符号化する
 */
int encode()
{
        int x = nodeX[0], y = nodeY[0];
        int code = P_INDEX(x, y);               /* 頭の位置 */
        int odir = 1;
        int node;

        for (node = 1; node < nNodes; node++) {
                int dx = -x;
                int dy = -y;
                int dir, reldir;
                dx += x = nodeX[node];
                dy += y = nodeY[node];
                dir = DIR(dx, dy);
                if (node == 1) {
                        code = code*N_DIRS + dir;
                }
                else {
                        /* 前の接続方向を基準とした接続の方位 (5/0/1) */
                        reldir = (DIR(dx, dy) + N_DIRS - odir) % N_DIRS;
                        if (reldir == 5) reldir = -1;
                        code = code*N_JUNC + reldir + 1;
                }
                odir = dir;
        }

        return code;
}

/*
 *      状態コードから状態を復元する
 */
void decode(int code)
{
        int pos, base, dir, node, x, y;

        base = N_DIRS;
        for (node = 2; node < nNodes; node++)
                base *= N_JUNC;

        for (node = 0; node < nNodes; node++)
                PIT(nodeX[node], nodeY[node]) = EMPTY;

        pos = code/base;
        x = nodeX[0] = pos/SIZE - RADIUS;
        y = nodeY[0] = pos%SIZE - RADIUS;
        PIT(x, y) = SNAKE(0);

        dir = 0;
        for (node = 1; node < nNodes; node++) {
                code %= base;
                if (node == 1) {
                        base /= N_DIRS;
                        dir = (dir + code/base) % N_DIRS;
                }
                else {
                        base /= N_JUNC;
                        dir = (dir + N_DIRS + code/base - 1) % N_DIRS;
                }
                nodeX[node] = x += dX[dir];
                nodeY[node] = y += dY[dir];
                PIT(nodeX[node], nodeY[node]) = SNAKE(node);
        }
}

/*
 *      残りステップ数を更新する --- 更新されたかどうかを返す
 */
int setVisited(int code, int rest)
{
        int u = sizeof(visitRest[0])*8/D_BITS;
        int i = code/u;
        int s = code%u*D_BITS;
        int mask = ((1 << D_BITS) - 1);
        int old = ((visitRest[i] >> s) & mask);

        if (rest > old) {
                visitRest[i] ^= ((old ^ rest) << s);
                return 1;
        }
        return 0;
}

/*
 *      状態の初期化
 */
void reset()
{
        memset((void *)visitRest, 0, sizeof(visitRest));
}

/*
 *      現在の状態からゴールまでのステップ数の下界を見積もって返す
 */
int minStep()
{
        int x = nodeX[0], y = nodeY[0];
        int head, second;

        /* 頭の節の移動に掛かるステップ数の下界を見積もる --- 1ステップで
        x, y は高々±1 しか変化しない。 また、x, y が同じ方向に同時に変化
        することはない */
        if (x >= 0 && y >= 0)
                head = x+y;
        else if (x <= 0 && y <= 0)
                head = -x-y;
        else {
                if (x < 0)
                        x = -x;
                if (y < 0)
                        y = -y;
                head = (x > y) ? x : y; 
        }

        /* 同様に2番目の節の移動に掛かるステップ数の下界を見積もる ---
        2番目の節は、最終的には原点(ゴール)の隣りに移動しなければならない */
        x = nodeX[1], y = nodeY[1];

        if (x >= 1 && y >= 1)
                second = x+y-2;
        else if (x <= -1 && y <= -1)
                second = -x-y-2;
        else {
                if (x < 0)
                        x = -x;
                if (y < 0)
                        y = -y;
                second = ((x > y) ? x : y) - 1;
        }

        if (second < 0)
                second = 0;

        return head + second;
}

/*
 *      個々の節の動きに関する可能性を列挙する
 *      複数の節が同時に動くと許されない動きとなる場合もあることに注意
 */
void enumerate(signed char dirs[MAX_NODES][3])
{
        int node;
        int f, b, d, num;

        /* 頭の節 */
        num = 0;
        f = DIR(nodeX[1] - nodeX[0], nodeY[1] - nodeY[0]);
        d = (f-1+N_DIRS) % N_DIRS;
        if (PIT(nodeX[0]+dX[d], nodeY[0]+dY[d]) != ROCK)
                dirs[0][num++] = d;
        d = (f+1) % N_DIRS;
        if (PIT(nodeX[0]+dX[d], nodeY[0]+dY[d]) != ROCK)
                dirs[0][num++] = d;
        dirs[0][num++] = -1;

        /* 中間の節 */
        for (node = 1; node < nNodes-1; node++) {
                num = 0;
                f = DIR(nodeX[node+1] - nodeX[node], nodeY[node+1] - nodeY[node]);
                b = DIR(nodeX[node-1] - nodeX[node], nodeY[node-1] - nodeY[node]);
                if ((f+2) % N_DIRS == b) {
                        d = (f+1) % N_DIRS;
                        if (PIT(nodeX[node]+dX[d], nodeY[node]+dY[d]) != ROCK)
                                dirs[node][num++] = d;
                }
                else if ((b+2) % N_DIRS == f) {
                        d = (b+1) % N_DIRS;
                        if (PIT(nodeX[node]+dX[d], nodeY[node]+dY[d]) != ROCK)
                                dirs[node][num++] = d;
                }
                dirs[node][num++] = -1;
        }

        /* 最後尾の節 */
        num = 0;
        b = DIR(nodeX[nNodes-2] - nodeX[nNodes-1],
                nodeY[nNodes-2] - nodeY[nNodes-1]);
        d = (b-1+N_DIRS) % N_DIRS;
        if (PIT(nodeX[nNodes-1]+dX[d], nodeY[nNodes-1]+dY[d]) != ROCK)
                dirs[nNodes-1][num++] = d;
        d = (b+1) % N_DIRS;
        if (PIT(nodeX[nNodes-1]+dX[d], nodeY[nNodes-1]+dY[d]) != ROCK)
                dirs[nNodes-1][num++] = d;
        dirs[nNodes-1][num++] = -1;
}

#define STAY    (-1)

int search(int depth, int limit)
{
        signed char dirs[MAX_NODES][3]; /* 個々の節の動きの可能性 */
        signed char moves[MAX_NODES];   /* 各節の動き (dirs の第2添字) */
        int code, node;

        /* ゴールしたかチェック */
        if (nodeX[0] == 0 && nodeY[0] == 0)
                return 1;

        /* 深さの上限をチェック  */
        if (depth + minStep() > limit)
                return 0;

        code = encode();

        /* 残りステップ数が以前より多い場合だけ考える */
        if (!setVisited(code, limit-depth))
                return 0;

        enumerate(dirs);

        for (node = 0; node < nNodes; node++) 
                moves[node] = STAY;

        /* この状態からの変化を調べる */
        while (1) {
                /* このステップの動きに関する次の可能性を生成する */
                for (node = 0; node < nNodes; node++) {
                        int n;

                        /* 次の節が動くのならこの節は動けない */
                        if (node+1 < nNodes && moves[node+1] != STAY)
                                continue;
                        if (dirs[node][++moves[node]] >= 0)
                                break;
                        /* この節に他の動きはない */
                        for (n = 0; n <= node; n++)
                                moves[n] = STAY;
                }

                /* このステップの動きに関しては他の可能性はない */
                if (node >= nNodes)
                        break;

                /* 動かしてみる */
                for (node = 0; node < nNodes; node++) {
                        int m = moves[node];

                        PIT(nodeX[node], nodeY[node]) = EMPTY;

                        if (m >= 0) {
                                int d = dirs[node][m];
                                nodeX[node] += dX[d];
                                nodeY[node] += dY[d];
                        }
                }

                /* PIT を更新しながら妥当性をチェック */
                for (node = 0; node < nNodes; node++) {
                        int x = nodeX[node], y = nodeY[node], d;

                        /* 節が領域外 */
                        if (x < -RADIUS || RADIUS < x || y < -RADIUS || RADIUS < y)
                                break;

                        /* 同じ窪みに複数の節がある */
                        if (PIT(x, y) != EMPTY)
                                break;

                        for (d = 0; d < N_DIRS; d++) {
                                int q = PIT(x+dX[d], y+dY[d]);

                                /* 直接つながっていない節が隣接している */
                                if (IS_SNAKE(q) && (q < node-1 || node+1 < q))
                                        break;
                        }

                        if (d < N_DIRS)
                                break;

                        PIT(x, y) = SNAKE(node);
                }

                /* うまく移動できていれば、次のステップへ進む */
                if (node >= nNodes) {
                        if (search(depth+1, limit))
                                return 1;
                }

                /* 状態を戻す */
                decode(code);
        }

        return 0;
}

int main()
{
        while (readData() > 0) {
                int d = minStep();

                reset();
                while (1) {
                        if (search(0, d))
                                break;
                        d++;
                }
                printf("%d\n", d);
        }
}