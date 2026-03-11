#include<stdio.h>
#define M 110

int main(void){
  int i;
  int n,num[M];
  int sign[8],light[8];

  for(;;){
    scanf("%d",&n);
    if(n==-1)
      break;

    for(i=0;i<n;i++)
      scanf("%d",&num[i]);

    for(i=0;i<8;i++)
      light[i]=0;

    for(i=0;i<n;i++){
      if(num[i]==0){
        printf("%d",sign[0]=(light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(!light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==1){
        printf("%d",sign[0]=(light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==2){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(!light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==3){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==4){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==5){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==6){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(!light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==7){
        printf("%d",sign[0]=(light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==8){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(!light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }else if(num[i]==9){
        printf("%d",sign[0]=(!light[0])?1:0); if(sign[0]) light[0]=!light[0];
        printf("%d",sign[1]=(!light[1])?1:0); if(sign[1]) light[1]=!light[1];
        printf("%d",sign[2]=(light[2])?1:0); if(sign[2]) light[2]=!light[2];
        printf("%d",sign[3]=(!light[3])?1:0); if(sign[3]) light[3]=!light[3];
        printf("%d",sign[4]=(!light[4])?1:0); if(sign[4]) light[4]=!light[4];
        printf("%d",sign[5]=(!light[5])?1:0); if(sign[5]) light[5]=!light[5];
        printf("%d",sign[6]=(!light[6])?1:0); if(sign[6]) light[6]=!light[6];
        printf("\n");
      }
    }
  }
  return 0;
}