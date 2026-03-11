#include<stdio.h>
#include<stdlib.h>
#include<string.h>
#include<math.h>
#define INF 10000
#define BalMax 3
#define debug 0

int DP[3][41];
int n,p[40],t[40];
char JDG[3]="aa";


void initDP(void){
  int i,j;
  for(i=0;i<3;i++){
    for(j=0;j<41;j++){
      DP[i][j]= INF;
    }
  }
}

int minimum(int a, int b){
  if(a<b)return a;
  else   return b;
}


int calc1(int num){
  int min;
  int dt,dx;
  int A,B,C;
  dt = t[num] - t[num-1];
  dx = p[num] + p[num-1];

  if(p[num]+4*p[num-1] > dt){
    A = INF;
  }
  else{
    A = DP[2][num-1] + dx;
  }

  if(p[num]+p[num-1]*3 > dt){
    B = INF;
  }  
  else{
    B = DP[1][num-1] + dx;
  }

  if(p[num]+p[num-1]*2 > dt){
    C = INF;
  }
  else{
    C = DP[0][num-1] + dx;
  }

  return minimum(A ,minimum(B ,C) );

}

int calc2(int bal, int num){
  int dt,dx; 
  
  dt = t[num] - t[num-1];//Ô·
  dx = abs(p[num] - p[num-1]);//£

  if(dx*(bal+1) > dt){//æêÈ¢
    return INF;
  }
  else{
    return DP[bal-1][num-1]+dx;
  }
}


int val(int bal, int num){
  int k;
  switch(bal){
  case 0://¨¢Ä©çÆèÉ¢­
    k = calc1(num);
    break;
  case 1://1±ÁÄéóÔ©ç»ÌÜÜæèÉ¢­
    k = calc2(bal,num);
    break; 
  case 2://2±ÁÄéóÔ©ç»ÌÜÜæèÉ¢­
    k = calc2(bal,num);
    break;
  }
  return k;
}

int solve(void){
  int i,j,min;
  if(p[0]>t[0]){//1ÂÚªÆêÈ¢ê
    strcpy(JDG,"NG");
    return 0+1;
  }
  else{         //êÂÚÍæêéê
    //úð
    DP[0][0]=p[0];
    min = p[0];

    for(j=1;j<n;j++){
      min = INF;
      for(i=0;i<BalMax;i++){
	DP[i][j] = val(i,j);//æéOÌÂi,ÉæéDÌjÔ
	if(DP[i][j]<min){
	  min = DP[i][j];
	}
      }
      if(min >= INF){//ÆêÈ©Á½
	strcpy(JDG,"NG");
	break;
      }
    }
    
    if(j==n){//Sæê½êifor[vðbreakÅ²¯È¢j
      strcpy(JDG,"OK");
      return min+p[n-1];
    }else{
      return j+1;
    }
  }
}

int main(void)
{
  int i,j,x;

  while(1){
    //t@CÇÝÝ
    scanf(" %d ", &n);
    if(n == 0)break;
    for(i=0;i<n;i++){
      scanf("%d %d",&p[i],&t[i]);
    }    
    //ú»
    initDP();

    if(debug){//fobOpoÍ
      printf("n=%d\n",n);
      for(i=0;i<n;i++){
	printf("p[%d]=%d,t[%d]=%d\n",i,p[i],i,t[i]);
      }
      putchar('\n');
      
    }
    
    x=solve();
    
    if(debug){ //fobOpoÍ
      printf("DP\n");
      for(j=0;j<3;j++){
	for(i=0;i<n;i++){
	  printf("%d ",DP[j][i]);
	}
	putchar('\n');
      }	
    }
    //¦ÌoÍ
    printf("%s %d\n",JDG ,x);
  }
    
  
  //fclose(fp);
  return 0;
}