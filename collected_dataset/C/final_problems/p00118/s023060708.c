#include <stdio.h>
#include <string.h>

char j[102][102];

int search(int c,int d,int e);

int main(void){
    while(1){
    int H,W,i,t,a,b=0;
    memset(j,0,sizeof(j));
    scanf("%d %d",&H,&W);
    if((H==0)&&(W==0))break;
    for(i=1;i<=H;i++){
            scanf("%s",&j[i][1]);
    };
    
    for(i=1;i<=H;i++){
        for(t=1;t<=W;t++){
            if(j[i][t]!=0){
                a=j[i][t];
                search(i,t,a);
                b++;
            };
        };
    };
    printf("%d\n",b);
        
    };
    return 0;
}

int search(int c,int d,int e){
    j[c][d]=0;
         if(j[c][d+1]==e){search(c,d+1,e);};
         if(j[c][d-1]==e){search(c,d-1,e);};
         if(j[c+1][d]==e){search(c+1,d,e);};
         if(j[c-1][d]==e){search(c-1,d,e);};
        return(0);
    };
