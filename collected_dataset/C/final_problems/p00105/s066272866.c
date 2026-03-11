#include <stdio.h>
#include <string.h>

typedef struct Word{
	char	word[31];
	int		page;
}	WORD;
WORD wordArray[100];

typedef int		BOOL;
#define TRUE	1;
#define FALSE	0;
BOOL is_smaller(WORD, int);

int main()
{
	int 	cnt1, cnt2;
	WORD	tempWORD;
	int		output_cnt = 0;
	char	last_print_word[31] = "";
	
	/*Input with order*/
	while(scanf("%s %d", tempWORD.word, &tempWORD.page) != EOF)
		for(cnt1 = 0; cnt1 < 100; cnt1++){
			if(wordArray[cnt1].page == 0){
				strcpy(wordArray[cnt1].word, tempWORD.word);
				wordArray[cnt1].page = tempWORD.page;
				break;
			}
			else if(is_smaller(tempWORD, cnt1)){
				for(cnt2 = 98; cnt2 >= cnt1; cnt2--){
					strcpy(wordArray[cnt2 + 1].word, wordArray[cnt2].word);
					wordArray[cnt2 + 1].page = wordArray[cnt2].page;
				}
				strcpy(wordArray[cnt1].word, tempWORD.word);
				wordArray[cnt1].page = tempWORD.page;
				break;
			}
		}
	
	/*Output*/
	while(wordArray[output_cnt].page != 0){
		if(strcmp(last_print_word, "") == 0){
			printf("%s\n", wordArray[output_cnt].word);
			strcpy(last_print_word, wordArray[output_cnt].word);
		}
		else if(strcmp(wordArray[output_cnt].word, last_print_word) != 0){
			printf("\n%s\n", wordArray[output_cnt].word);
			strcpy(last_print_word, wordArray[output_cnt].word);
		}
		else{
			printf(" ");
		}
		printf("%d", wordArray[output_cnt].page);
		output_cnt++;
	}
	printf("\n");
	
	return 0;
}

BOOL is_smaller(WORD tempWORD, int cnt)
{
	if(strcmp(tempWORD.word, wordArray[cnt].word) < 0){
		return TRUE;
	}
	else if(strcmp(tempWORD.word, wordArray[cnt].word) == 0){
		if(tempWORD.page < wordArray[cnt].page){
			return TRUE;
		}
		else{
			return FALSE;
		}
	}
	else{
		return FALSE;
	}
}