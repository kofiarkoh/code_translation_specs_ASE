#include <stdio.h>
#include <string.h>

char map[60][60], v[60][60];
int H, W;
int dr[]={-1, 0, 1, 0}, dc[]={0, -1, 0, 1};

int dfs(int r, int c, int k)
{
    char *t=&v[r][c];
    *t|=k;
    for(int i=0;i<4;++i) {
        int nr=r+dr[i];
        int nc=c+dc[i];
        if (nr<0 || nr>=H || nc<0 || nc>=W)
            *t|=4;
        else if (map[nr][nc]=='B')
            *t|=1;
        else if (map[nr][nc]=='W')
            *t|=2;
    }
    for(int i=0;i<4;++i) {
        int nr=r+dr[i];
        int nc=c+dc[i];
        if (nr<0 || nr>=H || nc<0 || nc>=W || map[nr][nc]!='.' || (*t&v[nr][nc])==*t) continue;
        dfs(nr, nc, *t);
    }
}

int main(void)
{
    while (scanf("%d%d", &W, &H), W) {
        int r, c, b, w;
        memset(map, 0, sizeof(map));
        memset(v, 0, sizeof(v));
        b=w=0;
        for(r=0;r<H;++r) scanf("%s", map[r]);

        for(r=0;r<H;++r) for(c=0;c<W;++c) {
            if (!v[r][c] && map[r][c]=='.') dfs(r, c, 0);
        }
        for(r=0;r<H;++r) for(c=0;c<W;++c) {
            b+=(v[r][c]==1 || v[r][c]==5);
            w+=(v[r][c]==2 || v[r][c]==6);
        }
        printf("%d %d\n", b, w);
    }
    return 0;
}