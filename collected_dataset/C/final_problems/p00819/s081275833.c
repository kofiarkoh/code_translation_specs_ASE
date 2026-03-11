#include <stdio.h>

void J(char *str, int length){//???????????????
	int i;
	for(i=length;i>0;i--){
		str[i]=str[i-1];
	}
	str[0]=str[length];
	str[length]=0;
	return;
}
void C(char *str, int length){//???????????????
	int i;
	str[length]=str[0];
	for(i=0;i<length;i++){
		str[i]=str[i+1];
	}
	str[length]=0;
	return;
}
void E(char *str, int length){
	int i;
	char temp[length/2];
	for(i=0;i<length/2;i++){
		temp[i]=str[i];
	}
	for(i=0;i<length/2;i++){
		str[i]=str[(length/2)+(length%2)+i];
	}
	for(i=0;i<length/2;i++){
		str[(length/2)+(length%2)+i]=temp[i];
	}
}
void A(char *str, int length){
	int i;
	char temp[length];
	for(i=0;i<length;i++){
		temp[i]=str[length-1-i];
	}
	for(i=0;i<length;i++){
		str[i]=temp[i];
	}
}
void P(char *str, int length){
	int i;
	for(i=0;i<length;i++){
		if(str[i]=='0'){
			str[i]='9';
		}else if('1'<=str[i]&&str[i]<='9'){
			str[i]--;
		}
	}
}
void M(char *str, int length){
	int i;
	for(i=0;i<length;i++){
		if(str[i]=='9'){
			str[i]='0';
		}else if('0'<=str[i]&&str[i]<='8'){
			str[i]++;
		}
	}
}

void solve(){
	int i,length;
	char str[26];
	char route[7];

	scanf("%s",route);
	scanf("%s",str);

	for(i=0;str[i];i++);
	length=i;
	for(i=0;route[i];i++);
	for(i--;i>=0;i--){
		switch(route[i]){
			case 'J':
				J(str,length);
				break;
			case 'C':
				C(str,length);
				break;
			case 'E':
				E(str,length);
				break;
			case 'A':
				A(str,length);
				break;
			case 'P':
				P(str,length);
				break;
			case 'M':
				M(str,length);
				break;
		}
	}
	printf("%s\n",str);
}

int main(void){
	int n;
	scanf("%d",&n);
	for(;n;n--){
		solve();
	}
	return 0;
}