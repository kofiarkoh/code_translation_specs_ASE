#include <stdio.h>
#include <stdlib.h>


// ?????¬??????stdin ????????\????????????

char *garden;
int h,w;


int dfs(int x,int y, char c){
    if(x < 0 || y < 0 || x >= w | y >= h){
        return 0;
    }

    if(c == '.')
        return 0;

    if(c == garden[y*h+x]){
        garden[y*h+x]= '.';
        dfs(x+1,y, c);
        dfs(x-1,y, c);
        dfs(x,y-1, c);
        dfs(x,y+1, c);
    }
    return 0;
}

void solve(){

    int x,y,sum=0;
    char c;

    // ??§?????¢?´¢????????????
    for(y=0;y<h;y++){
        for(x=0;x<w;x++){
            c = garden[y*h+x];
            if(c != '.'){
                dfs(x,y,c);
                sum++;
            }
        }
    }
    free(garden);
    printf("%d\n", sum);


}

int main()
{
    int i;
    while(1){
        scanf("%d %d", &h,&w);
        if(h ==0 && w==0)
            return 0;;

        garden = malloc(sizeof(char)*h*w);

        for(i=0;i<h;i++){
            scanf("%s", &garden[i*w]);
        }

        solve();
    }
    return 0;
}