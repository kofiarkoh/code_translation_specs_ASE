#include<stdio.h>
int main(void){
    while(1){
        char gengou[10]; int y,m,d;
        scanf("%s",gengou);
        if(gengou[0]=='#') break;
        scanf("%d%d%d",&y,&m,&d);
        if((y==31&&m>=5)||y>=32){
            printf("? %d %d %d\n",y-30,m,d);
        }
        else{
            printf("%s %d %d %d\n",gengou,y,m,d);
        }
    }
}

