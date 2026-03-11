#include <stdio.h>

struct station{
        char name[80];
        int h1;
        int m1;
        int h2;
        int m2;
        char next[80];
        int trance;
};

int main(void){
        struct station st[10];
        int n,t,que = 0;
        int i;
        int che[10] = {};

        scanf("%d %d",&n,&t);
        for(i = 0; i < n; i++)
                scanf("%d:%d %s %d:%d %s",&st[i].h1,&st[i].m1,st[i].name,&st[i].h2,&st[i].m2,st[i].next);

        for(i = 0; i < n-1; i++){
                if((st[i+1].h1 * 60 +st[i+1].m1 -st[i].h2 * 60 - st[i].m2) >= t){
                        st[i+1].trance = (st[i+1].h1 * 60 +st[i+1].m1 -st[i].h2 * 60 - st[i].m2);
                        che[que] = i+1;
                        que ++;
                }
        }
        printf("%d\n",que);
        for(i = 0; i < que; i++)
                printf("%s %d\n",st[che[i]].name,st[che[i]].trance);

        return 0;
}
