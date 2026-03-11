#include<stdio.h>
int f(char d[7][7]){
  if(d[2][2]!='.'){
    if(d[2][4]=='.'){
      if(d[4][2]=='.')return 5;
      return 6;
    }
    if(d[1][3]=='.')return 4;
    if(d[4][2]!='.')return 8;
    return 9;
  }
  else{
    if(d[3][3]!='.'){
      if(d[4][2]!='.')return 2;
      return 3;
    }
    if(d[1][3]!='.')return 7;
    return 1;
  }
}   
int main(){
  char d[21][58];
  int w[2]={0,29};
  int Y[]={ 0, 7, 7, 7, 7,14};
  int X[]={ 7, 0, 7,14,21, 7};
  int y[4][5]={
    {1,2,2,3,4},
    {3,4,2,3,2},
    {3,2,4,3,4},
    {2,3,4,4,5}
  };   
  int x[4][5]={
    {3,2,4,3,4},
    {1,2,2,3,4},
    {5,4,4,3,2},
    {2,3,4,2,3}
  };
  int s[2][6],i,j,k,c,t;
  while(scanf("%s",d[0]),d[0][0]!='0'){
    for(i=0;i<20;i++){
      scanf("%s",d[i+1]);
    }
    for(i=0;i<2;i++){
      for(j=0;j<6;j++){
	char m[7][7];
	for(k=0;k<5;k++){
	  if(j%2){
	    t=j/2+1;
	    m[y[0][k]][6-x[0][k]]=d[Y[j]+y[t][k]][w[i]+X[j]+x[t][k]];
	  }
	  else{
	    m[y[0][k]][6-x[0][k]]=d[Y[j]+y[0][k]][w[i]+X[j]+x[0][k]];
	  }
	}
	s[i][j]=f(m);//printf("%d ",s[i][j]);
      }
    }//printf("\n");
    for(i=0;i<6;i++){
      for(j=i;j;j--){
	for(k=0;k<2;k++){
	  if(s[k][j]>s[k][j-1]){
	    t=s[k][j];
	    s[k][j]=s[k][j-1];
	    s[k][j-1]=t;
	  }
	}
      }
    }
    for(i=c=0;i<6;i++){
      if(s[0][i]>s[1][i])c++;
      if(s[0][i]<s[1][i])c--;
    }
    if(c<0)printf("LOW\n");
    else   printf("HIGH\n");
  }
  return 0;
}