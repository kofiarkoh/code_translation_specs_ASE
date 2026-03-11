#include <stdio.h>
#include <math.h>

typedef struct{
    int x, y;
}point;

point c[21];
point hero;
point demon;

int n;
int c_len[21];
int done[21];

int solve(int no, int day)
{
    int i;
    //if(no)
    //    printf("day %d: got crystal no %d\n", day, no);

    for(i = 1; i <= n; i++)
        if(!done[i])
            break;

    if(i == (n + 1))
        return 1;

    for(i = 1; i <= n; i++)
        if(c_len[i] <= day * day && !done[i])
            return 0;

    int ret = 0;
    for(i = 1; i <= n; i++){
        if(done[i])
            continue;

        int dx = c[i].x - c[no].x;
        int dy = c[i].y - c[no].y;
        double len = dx * dx + dy * dy;
        double plen = sqrt(c_len[i]);
        plen -= day;
        len = sqrt(len);

        //printf("%d to %d: %lf\n", no, i, len);
        //printf("poison circle to %d: %lf\n", i, plen);

        if(len >= plen)
            continue;
        double temp = fmod(plen, 1);

        int take;
        if(temp <= 1E-10)
            take = (int)len;
        else
            take = (int)len + 1;

        done[i] = 1;
        ret = solve(i, day + take);
        done[i] = 0;

        if(ret == 1)
            break;
    }

    return ret;
}

int main(void)
{
    while(1){
        scanf("%d %d %d %d %d", &n, &(hero.x), &(hero.y), &(demon.x), &(demon.y));
        if(n == 0 && hero.x == 0 && hero.y == 0 && demon.x == 0 && demon.y == 0)
            break;

        c[0].x = hero.x;
        c[0].y = hero.y;

        int i;
        for(i = 1; i <= n; i++){
            int dx, dy;
            done[i] = 0;
            scanf("%d %d", &(c[i].x), &(c[i].y));
            dx = demon.x - c[i].x;
            dy = demon.y - c[i].y;
            c_len[i] = dx * dx + dy * dy;
        }

        int res = solve(0, 0);
        if(res)
            printf("YES\n");
        else
            printf("NO\n");
    }

    return 0;
}