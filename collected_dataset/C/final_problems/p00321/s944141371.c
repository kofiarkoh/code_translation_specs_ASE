#include<stdio.h>
#include<string.h>
struct DOBERMAN{
char c[61],s[61];
int g;
};
struct DOBERMAN doberman[1000000]={0},q[1000000]={0},t;
int main(){
int i,j,n,f,m,u=0,o=0,r,l,b=0,jc,jk,js;
static int p[1000000]={0};
char d[61][61];
scanf("%d%d",&n,&f);
for(i=0;i<n;i++){
	scanf("%d",&m);
	for(j=0;j<m;j++){
		scanf("%s",d[j]);
	}
	for(j=0;j<m;j++){
		for(r=j+1;r<m;r++){
			o=0;
			//printf("%s %s\n",d[j],d[r]);
			for(l=0;l<u;l++){
				if((strcmp(d[j],doberman[l].c)==0&&strcmp(d[r],doberman[l].s)==0)||(strcmp(d[r],doberman[l].c)==0&&strcmp(d[j],doberman[l].s)==0)){
					//printf("%d:%d\n%d:%d\n",strcmp(d[j],doberman[l].c),strcmp(d[r],doberman[l].s),strcmp(d[r],doberman[l].c),strcmp(d[j],doberman[l].s));
					o=1;
					doberman[l].g++;
					if(doberman[l].g>=f&&p[l]==0){
						q[b]=doberman[l];
						p[l]=1;
						b++;
					}
					break;
				}
			}
			if(o==0){
				//printf("MorishitaSHINE\n");
				js=0;
				if(strlen(d[j])>strlen(d[r])){
					jk=strlen(d[r]);
				}
				else{
					jk=strlen(d[j]);
				}
				for(jc=0;jc<jk;jc++){
					if(d[j][jc]-'a'>d[r][jc]-'a'){
						js=1;
						break;
					}
					if(d[j][jc]-'a'<d[r][jc]-'a'){
						js=2;
						break;
					}
				}
				if(js==1){
					strcpy(t.c,d[r]);
					strcpy(t.s,d[j]);
					t.g=doberman[u].g+1;
				}
				else{
				strcpy(t.c,d[j]);
				strcpy(t.s,d[r]);
				t.g=doberman[u].g+1;
				}
				doberman[u]=t;
				//printf("%s %s %d\n",doberman[u].c,doberman[u].s,doberman[u].g);
				if(doberman[u].g>=f&&p[u]==0){
					q[b]=doberman[u];
					p[u]=1;
					b++;
				}
				u++;
			}
		}
	}
}
for(i=0;i<b;i++){
	for(j=i+1;j<b;j++){
		js=0;
				if(strlen(q[i].c)>strlen(q[j].c)){
					jk=strlen(q[j].c);
				}
				else{
					jk=strlen(q[i].c);
				}
		for(jc=0;jc<jk;jc++){
			if(q[i].c[jc]-'a'>q[j].c[jc]-'a'){
				js=1;
				break;
			}
			if(q[i].c[jc]-'a'<q[j].c[jc]-'a'){
				js=2;
				break;
			}
		}
		if(js==0){
				if(strlen(q[i].s)>strlen(q[j].s)){
					jk=strlen(q[j].s);
				}
				else{
					jk=strlen(q[i].s);
				}
		for(jc=0;jc<jk;jc++){
			if(q[i].s[jc]-'a'>q[j].s[jc]-'a'){
				js=1;
				break;
			}
			if(q[i].s[jc]-'a'<q[j].s[jc]-'a'){
				js=2;
				break;
			}
		}
		}
		if(js==1){
			t=q[i];
			q[i]=q[j];
			q[j]=t;
		}
	}
}
printf("%d\n",b);
for(i=0;i<b;i++){
	printf("%s %s\n",q[i].c,q[i].s);
}
return 0;
}
