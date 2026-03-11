
#include <stdio.h>
#include <stdlib.h>

int mas[3][3];
int cpy[3][3];





void copy(int m[3][3], int t[3][3]) {
    int i, j;
    
    for(i = 0; i < 3; i++)
        for(j = 0; j < 3; j++) m[i][j] = t[i][j];
}

int calc() {
    int i, sum = 0;
    for(i = 0; i < 9; i++) {
        int v = mas[i/3][i%3];
        if(v) {
            v--;
            sum += abs(i/3-v/3)+abs(i%3-v%3);
        }
    }
    return sum;
}

int dx[] = {0, 1, 0, -1};
int dy[] = {1, 0, -1, 0};

void swap(int *a, int *b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

int is_in(int x, int y) {
    return 0<=x&&x<3&&0<=y&&y<3;
}

int dfs(int x, int y, int p, int dep, const int lim) {
    int i, h = calc();
    if(h == 0) return dep;
    if(dep+h > lim) return -1;
    
    for(i = 0; i < 4; i++) {
        int nx = x+dx[i], ny = y+dy[i], res;
        if(~p && i == (p+2)%4) continue;
        if(!is_in(nx, ny)) continue;
        swap(&mas[y][x], &mas[ny][nx]);
        res = dfs(nx, ny, i, dep+1, lim);
        if(~res) return res;
        swap(&mas[y][x], &mas[ny][nx]);
    }
    
    return -1;
}

void get_empty(int *x, int *y) {
    int i, j;
    for(i = 0; i < 3; i++) {
        for(j = 0; j < 3; j++) {
            if(!mas[i][j]) {
                *x = j;
                *y = i;
                return;
            }
        }
    }
}

int solve() {
    int x, y, lim, ans;
    get_empty(&x, &y);
    for(lim = 0; lim < 50; lim++) {
        copy(cpy, mas);
        if((ans = dfs(x, y, -1, 0, lim)) != -1) return ans;
        copy(mas, cpy);
    }
    return -1;
}

int main() {
    int i, j;
    for(i = 0; i < 3; i++)
        for(j = 0; j < 3; j++)
            scanf("%d", &mas[i][j]);
    printf("%d\n", solve());
    return 0;
}

