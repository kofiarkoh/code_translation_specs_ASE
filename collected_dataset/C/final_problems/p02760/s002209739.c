#include <stdio.h>
int a[101],b[10];
int main(){
  int n,c,i;

  for(i=0;i<9;i++){
	   scanf("%d",&n);
	    a[n]=i;
  }
  scanf("%d",&n);
  for(i=0;i<n;i++){
  	scanf("%d",&c);
  	b[a[c]]=1;
  }
  if((b[0]*b[1]*b[2])==1||(b[3]*b[4]*b[5])==1||(b[6]*b[7]*b[8])==1||(b[0]*b[3]*b[6])==1||(b[1]*b[4]*b[7])==1||(b[2]*b[5]*b[8])==1||(b[0]*b[4]*b[8])==1||(b[2]*b[4]*b[6])==1){
  	printf("Yes");
  }else{
  	printf("No");
  }
  return 0;
}
