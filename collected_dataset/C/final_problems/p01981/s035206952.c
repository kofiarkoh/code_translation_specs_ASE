#include <stdio.h>
#include <string.h>

int main() {
    int y,m,d;
    char g[10];
    
    while(1){
        scanf("%s%d%d%d",g,&y,&m,&d);
        if(strcmp(g, "#")==0) break;
        if((y==31 && m>4) || y>31)
            printf("? %d %d %d\n",y-30,m,d);
        else
            printf("%s %d %d %d\n",g,y,m,d);
    }
    return 0;
}
