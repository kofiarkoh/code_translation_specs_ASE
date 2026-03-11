#include <stdio.h>
#include <memory.h>

int main(void){
    int map[5],line[5],sign,i,o;
    for(;sign=0,memset(map,0,sizeof(map)),memset(line,0,sizeof(line)),scanf("%d",&line[0]),map[line[0]]++,line[0];){
        for(i=1;i<5;i++){
            scanf("%d",&line[i]);
            map[line[i]]++;
        }
        for(i=1;i<4;i++){
            if(map[i] == 0){
                sign++;
                for(o=0;o<5;o++){
                    if(((i+1)==4?1:i+1) == line[o])
                        printf("1\n");
                    else
                        printf("2\n");
                }
            }
        }
        if(sign == 0)
            for(i=0;i<5;i++)
                printf("3\n");
    }
    return 0;
}