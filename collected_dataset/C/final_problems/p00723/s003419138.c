#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>
#include <limits.h>


int main()
{
	int n,m;
	int i,j,k,l;
	int ans;

	scanf("%d",&n);

	for(i=0;i<n;i++)
	{
		char a[73];
		char c[1000][73];
		ans=1;
		scanf("%s",a);
		strcpy(c[1],a);

		//文字列を作成する
		for(j=1;j<strlen(a);j++)
		{
			int count=0;
			char f[73]={0};
			char b[73]={0};
			char mf[73]={0};
			char mb[73]={0};
			for(k=0;k<j;k++){f[k]=a[k];}
			for(k=j;k<strlen(a);k++){b[k-j]=a[k];}
			for(k=0;k<strlen(f);k++){mf[strlen(f)-k-1]=f[k];}
			for(k=0;k<strlen(b);k++){mb[strlen(b)-k-1]=b[k];}

			//文字列を比較する
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,f);strcat(x,mb);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,mf);strcat(x,mb);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,mf);strcat(x,b);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,f);strcat(x,mb);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,b);strcat(x,f);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,mb);strcat(x,f);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,mb);strcat(x,mf);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}

			count=0;
			for(k=1;k<=ans;k++){	
				char x[73]={0};
				strcat(x,b);strcat(x,mf);
				if(strcmp(c[k],x)!=0){count++;}
				if(count==ans){strcpy(c[ans+1],x);ans++;break;}}
		}
		printf("%d\n",ans);

		}
	return 0;
}