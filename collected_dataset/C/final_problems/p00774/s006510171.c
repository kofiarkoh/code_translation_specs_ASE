#include <stdio.h>

int board[10][5];
int H;
int solve(){
    int point=0,cont;
    int i,j,k;
    int flag;
    while(1){
        flag=0;
        cont=0;
        for(i=0;i<H;i++){
            for(j=0;j<4;j++){
                if(board[i][j]==board[i][j+1]&&board[i][j]){
                    cont++;
                    if(j+1==4&&cont>1){
                        for(k=0;k<cont+1;k++){
                            point+=board[i][4-k];
                            board[i][4-k]=0;
                        }
                        cont=0;
                        flag=1;
                    }
                }else{
                    if(cont<2){
                        cont=0;
                    }else{
                        for(k=0;k<cont+1;k++){
                            point+=board[i][j-k];
                            board[i][j-k]=0;
                        }
                        cont=0;
                        flag=1;
                    }
                }
            }
            cont=0;
        }
        for(i=0;i<5;i++){
            for(j=H-1;j>=0;j--){
                if(board[j][i]==0){
                    cont=0;
                    while(board[j-cont][i]==0){
                        cont++;
                        if(j-cont<0) break;
                    }
                    if(j-cont<0) break;
                    board[j][i]=board[j-cont][i];
                    board[j-cont][i]=0;
                }
            }
        }
        /**
        for(i=0;i<H;i++){
            printf("%d %d %d %d %d\n",board[i][0], board[i][1], board[i][2], board[i][3], board[i][4]);
        }
        printf("\n");
        **/
        if(flag==0){
            break;
        }
    }
    return point;
}

int main(void){
    int i;
    while(scanf("%d", &H),H){
        for(i=0;i<H;i++){
            scanf("%d %d %d %d %d", &board[i][0], &board[i][1], &board[i][2], &board[i][3], &board[i][4]);
        }
        printf("%d\n",solve());
    }
}