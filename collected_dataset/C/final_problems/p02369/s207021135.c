#include <stdio.h>
#include <stdlib.h>

#define MAXV 100
#define MAXE 1000

int v, e;
int a = 0;
int *visited;
int *s, *t;

void resultSearch(int);

int main(){

        int i;

        scanf("%d %d", &v, &e);

        if(e < 0 || MAXE < e || v < 1 || MAXV < v)return 0;

        s = (int*)malloc(e*sizeof(int));
        t = (int*)malloc(e*sizeof(int));


        visited = (int*)malloc(v*sizeof(int));

        for(i = 0;i < e;i ++)
                {
                        scanf("%d %d", &s[i], &t[i]);
                }

        for(i = 0;i < v;i ++)
                {
                        visited[i] = 0;
                }

        for(i = 0;i < v;i ++)
                {
                        if(a == 1)
                                {
                                        break;
                                }else if(visited[i]  == 0 && a == 0)
                                {
                                        resultSearch(i);
                                }
                }

        printf("%d\n", a);

        free(s);
        free(t);
        free(visited);

        return 0;
}

void resultSearch(int v){
        int i;

        if(visited[v] == 1){
                a = 1;
        }else if(visited[v] == 0){
                visited[v] = 1;

                for(i = 0; i < e; i ++)
                {
                        if(s[i] == v)resultSearch(t[i]);
                }
                visited[v] = 2;
        }
}
