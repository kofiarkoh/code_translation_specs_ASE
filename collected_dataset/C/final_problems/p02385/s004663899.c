#include <stdio.h>
  
int main(void){
    int i,j,k,a,b,dice[2][7],
        side[5] = {2,3,5,4,2},
        n = 0;
      
    for(i=0;i<2;i++)
      for(j=1;j<7;j++)
        scanf("%d", &dice[i][j]);
     
    for(i=0;i<6;i++){
      if(dice[0][1]==dice[1][1])
        for(j=0;j<4;j++){
 
  //一致判定
          for(k=1;k<7;k++)
            if(dice[0][k]!=dice[1][k]) break;
  
          if(k==7){
            printf("Yes\n");
            return 0;
          }
 
  //スピン
          dice[1][0] = dice[1][2];
          dice[1][2] = dice[1][3];
          dice[1][3] = dice[1][5];
          dice[1][5] = dice[1][4]; 
          dice[1][4] = dice[1][0]; 
 
       
      }
          
       //回転  
      
      dice[1][0] = dice[1][1];
          
      switch(i){
                  
          case 2:
              dice[1][1] = dice[1][3];
              dice[1][3] = dice[1][6];
              dice[1][6] = dice[1][4];
              dice[1][4] = dice[1][0]; 
              break;
              
          default:
              dice[1][1] = dice[1][2];
              dice[1][2] = dice[1][6];
              dice[1][6] = dice[1][5];
              dice[1][5] = dice[1][0]; 
              break;
         
          }
    }
 
    printf("No\n");
    return 0;
}
