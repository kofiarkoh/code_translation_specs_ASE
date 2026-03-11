#include <stdio.h>

int h,w,b[400][400],c[400][400],x[160000],y[160000];
long count(int, int),p;

long saki(int i, int j){
   long m;

   c[i][j]=1;m=0;
   x[p]=i;y[p]=j;p++;
   if (i>0 && b[i-1][j]==1 && c[i-1][j]==0) m+=count(i-1,j);
   if (i<h-1 && b[i+1][j]==1 && c[i+1][j]==0) m+=count(i+1,j);
   if (j>0 && b[i][j-1]==1 && c[i][j-1]==0) m+=count(i,j-1);
   if (j<w-1 && b[i][j+1]==1 && c[i][j+1]==0) m+=count(i,j+1);
   return m+1;
}

long count(int i, int j){
   long m;

   c[i][j]=1;m=0;
   x[p]=i;y[p]=j;p++;
   if (i>0 && b[i-1][j]==0 && c[i-1][j]==0) m+=saki(i-1,j);
   if (i<h-1 && b[i+1][j]==0 && c[i+1][j]==0) m+=saki(i+1,j);
   if (j>0 && b[i][j-1]==0 && c[i][j-1]==0) m+=saki(i,j-1);
   if (j<w-1 && b[i][j+1]==0 && c[i][j+1]==0) m+=saki(i,j+1);
   return m;
}

void clear(int i, int j){
   c[i][j]=0;
   if (i>0 && c[i-1][j]==1) clear(i-1,j);
   if (i<h-1 && c[i+1][j]==1) clear(i+1,j);
   if (j>0 && c[i][j-1]==1) clear(i,j-1);
   if (j<w-1 && c[i][j+1]==1) clear(i,j+1);
}

int main(){
   long n,k;
   int i,j;
   char s[401];
    
   scanf("%d %d",&h,&w);
   for (i=0;i<h;i++) {
      scanf("%s",s);
      for (j=0;j<w;j++) {
         if (s[j]=='#') b[i][j]=1;
         else           b[i][j]=0;
         c[i][j]=0;
      }
   }
   p=n=0;
   for (i=0;i<h;i++)
      for (j=0;j<w;j++) {
         if (b[i][j]) n+=count(i,j);
         for (k=0;k<p;k++) c[x[k]][y[k]]=0;
         p=0;
      }
   printf("%ld\n",n);
   return 0;
}
