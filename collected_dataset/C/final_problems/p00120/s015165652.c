#include<stdio.h>
#include<math.h>
#include<string.h>
double f(int a,int b){return sqrt((a+b)*(a+b)-(a-b)*(a-b));}
int nxpt(char *s){
  int i,l=strlen(s),max=0,f[260]={0};
  for(l--;l+1&&max<=s[l];l--)f[max=s[l]]++;
  if(l<0)return 0;
  f[s[l]]++;
  for(i=s[l]+1;f[i]==0;i++);
  f[s[l]=i]--;
  for(i=0;s[++l];f[s[l]=i]--){
    while(f[i]==0)i++;
  }
  return 1;
}
int main(){
  char s[100010],a[20];
  int i,r,n[20],m;
  double d[30],l;
  while(fgets(s,100000,stdin)!=NULL){
    for(i=r=m=0;s[i];i++){
      if('0'<=s[i]&&s[i]<='9')m=m*10+s[i]-'0';
      else{
	a[r]=r+1;
	n[r++]=m;
	m=0;
      }
    }
    a[r-1]=0;
    //for(i=0;i<r-1;i++)printf("%d\n",a[i]);
    do{//printf("\n");
      for(i=l=0;i+2<r;i++){//printf("%d %d\n",n[a[i]],n[a[i+1]]);
	d[i]=f(n[a[i]],n[a[i+1]]);
	l+=d[i];//printf("%f ",l);
      }//printf("%f ",l+n[1]+n[r-1]);
      if(l+n[a[0]]+n[a[r-2]]>n[0])goto END;
      for(i=0;i+3<r;i++){
	if(d[i]+d[i+1]<f(n[a[i]],n[a[i+2]]))goto END;
      }
      printf("OK\n");
      goto NEXT;
    END:;
    }while(nxpt(a));
    printf("NA\n");
  NEXT:;
  }
  return 0;
}