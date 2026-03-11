#include <stdio.h>
#include <memory.h>
int map[15][15],z;
void set(int x,int y,int num){
    if(num>z*z)
        return;
    do{
        if(x<0||x>=z)
            x<0?x+=z:(x-=z);
        if(y<0||y>=z)
            y<0?y+=z:(y-=z);
        if(map[y][x]){
            x-=1;
            y+=1;
        }
    }while(x<0||y<0||x>=z||y>=z||map[y][x]);
    map[y][x]=num;
    set(x+1,y+1,num+1);
}
int main(void){
    int i,o;
    for(;memset(map,0,sizeof(map)),scanf("%d",&z),z;){
        set(z/2,z/2+1,1);
        for(i=0;i<z;i++)
            for(o=0;o<z;o++)
                printf("%4d%s",map[i][o],o==z-1?"\n":"");
    }
    return 0;
}