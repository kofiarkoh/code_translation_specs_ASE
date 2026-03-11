#include<stdio.h>
#include<string.h>
int main(){
  char s1[70],s2[70],s3[70],na[1000][70],s[71];
  int st[1010],n,m,i,j,r,p[1010]={-1};
  while(scanf("%d %d",&n,&m),n||m){
    scanf("\n");
    st[0]=-1;
    for(j=0;j<n;j++){
      fgets(s,71,stdin);
      for(i=0;s[i]==' ';i++);
      st[i+1]=j;
      p[j]=st[i];//printf("%s\n",s+i);
      for(r=0;s[i]-10;i++)na[j][r++]=s[i];
      na[j][r]=0;
    }//printf("a\n");
    //for(i=0;i<n;i++)printf("%d\n",p[i]);
    while(m--){
      scanf("%s %*s %*s %s %*s %s",s1,s2,s3);//printf("%s\n",s3);
      s3[strlen(s3)-1]=0;
      for(i=0;strcmp(na[i],s1);i++);//printf("%d\n",i);
      for(j=0;strcmp(na[j],s3);j++);//printf("%d\n",j);
      if(s2[0]=='c'){
	if(j==p[i])printf("True\n");
	else       printf("False\n");
      }
      if(s2[0]=='a'){
	for(;j+1;j=p[j]){
	  if(i==j)break;
	}
	if(i==j)printf("True\n");
	else    printf("False\n");
      }
      if(s2[0]=='s'){
	if(p[i]==p[j])printf("True\n");
	else          printf("False\n");
      }
      if(s2[0]=='p'){
	if(i==p[j])printf("True\n");
	else       printf("False\n");
      }
      if(s2[0]=='d'){
	for(;i+1;i=p[i]){
	  if(i==j)break;
	}
	if(i==j)printf("True\n");
	else    printf("False\n");
      }
    }
    printf("\n");  
  }
  return 0;
}