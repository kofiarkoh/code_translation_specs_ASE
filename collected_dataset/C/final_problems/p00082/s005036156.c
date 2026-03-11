#include<stdio.h>

int main(void){
  int i,j;
  int p[8];
  int c[16];
  int curS,minS,minP;

  for(i=0;i<16;i++)
    if(i%2==0)
      c[i]=1;

  c[1]=2;
  c[3]=2;
  c[5]=4;
  c[7]=4;
  c[9]=2;
  c[11]=2;
  c[13]=4;
  c[15]=4;

  for(;scanf("%d%d%d%d%d%d%d%d",
        &p[0],&p[1],&p[2],&p[3],
        &p[4],&p[5],&p[6],&p[7])==8;){

    for(j=0;j<8;j++)
      if((p[j]-c[j])>0)
        curS+=p[j]-c[j];

    minS=curS;

    for(i=1;;){
      curS=0;

      for(j=0;j<8;j++)
        if((p[j]-c[i+j])>0)
          curS+=(p[j]-c[i+j]);

      if(minS>curS){
        minS=curS;
        minP=i;
      }

      if(i==1) i=7;
      else if(i==7) i=3;
      else if(i==3) i=5;
      else if(i==5) i=8;
      else if(i==8) i=6;
      else if(i==6) i=2;
      else if(i==2) i=4;
      else if(i==4) break;
    }

    for(i=minP;i<(minP+8);i++){
      printf("%d",c[i]);
      if(i!=(minP+7))
        printf(" ");
    }
    printf("\n");
  }
  return 0;
}