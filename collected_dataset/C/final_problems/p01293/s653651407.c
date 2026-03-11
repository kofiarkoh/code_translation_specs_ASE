#include<stdio.h>
int f(char *a,char *b,char c,char d){
  int n,m;
  char w[15]={"23456789TJQKA"};
  for(n=0;a[0]-w[n];n++);
  for(m=0;b[0]-w[m];m++);//printf("%2d %c %2d %c\n",n,a[1],m,b[1]);
  if(a[1]==b[1])return n>m?1:0;
  if(a[1]==c)return 1;
  if(b[1]==c)return 0;
  if(a[1]==d)return 1;
  if(b[1]==d)return 0;
  return n>m?1:0;
}  
int main(){
  char s[4][15][2],c,m;
  int i,j,n,p,b[5];
  while(scanf(" %c",&c),c-'#'){
    for(i=0;i<4;i++){
      for(j=b[i]=0;j<13;j++)scanf("%s",s[i][j]);
    }
    for(j=p=0;j<13;j++){
      m=s[p][j][1];
      for(i=0;i<4;i++){
	if(f(s[i][j],s[p][j],c,m))p=i;
      }//printf("%d  ",p);
      b[p]++;
    }//printf(" %d %d\n",b[0]+b[2],b[1]+b[3]);
    if  (b[0]+b[2]>b[1]+b[3])printf("NS %d\n",b[0]+b[2]-6);
    else printf("EW %d\n",b[1]+b[3]-6); 
  }
  return 0;
}
  