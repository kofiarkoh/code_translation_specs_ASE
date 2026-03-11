/*
 * Fast UNION(サイズによるUNION) + 経路圧縮
 * (入門データ構造とアルゴリズム(オライリー)8章)
 */
#include <stdio.h>

#define N 100000

/* 互いに素な集合を木で表す配列: */
/* 負の値: この要素は根であり、値はその木の要素数*(-1) */
/* 正の値: 親要素のインデックス */
int S[N+1];

/* 各場所に置いてある数字 */
int P[N+1];

void makeset(int size) {
    int i;
    /* 自身のみを含む1要素の集合に属するように初期化 */
    for (i = 1; i <= size; i++) {
        S[i] = -1; /* 根であり要素数は1 */
    }
}

int find(int x) {
    if (S[x] < 0) {
        return x;
    }

    int r = find(S[x]);
    S[x] = r;

    return r;
}

void merge(int x, int y) {
    int root1 = find(x);
    int root2 = find(y);

    if (root1 == root2) {
        return;
    }

    /* 要素数が少ない方を子にする。*/
    /* 要素数を-1倍して格納しているので不等号の向きに注意 */
    if (S[root2] < S[root1]) {
        /** root1をroot2の子にする */
        S[root2] += S[root1];
        S[root1] = root2;
    } else {
        /** root2をroot1の子にする */
        S[root1] += S[root1];
        S[root2] = root1;
    }
}

int main()
{
    int i;
    int n, m;
    int x, y;
    int r;

    scanf("%d %d", &n, &m);

    makeset(n);

    for (i = 1; i <= n; i++) {
        scanf("%d", &P[i]);
    }

    for (i = 1; i <= m; i++) {
        scanf("%d %d", &x, &y);
        merge(x, y);
    }

    r = 0;
    for (i = 1; i <= n; i++) {
        if (find(i) == find(P[i])) {
            r++;
        }
    }

    printf("%d\n", r);
    return 0;
}
