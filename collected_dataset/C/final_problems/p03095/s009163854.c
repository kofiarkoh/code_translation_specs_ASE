#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define TRUE 1
#define FALSE 0

typedef struct _words{
	char *s;
	int size;
} words;

void plus_binary_counter(int *s, int size) {
	int i;
	for ( i = 0; i < size; i++) {
		if (s[i] == 0) {
			s[i] = 1;
			break;
		}
		else if (s[i] == 1) {
			s[i] = 0;
		}
	}
}

int is_doubled(char *a_word) {
	if (strlen(a_word) > 27) {
		return FALSE;
	}
	words alphabets;
	alphabets.s = (char *)malloc(sizeof(char) * 27);
	alphabets.s[0] = a_word[0];
	alphabets.size = 1;
	for (int i = 1; i < strlen(a_word); i++) {
		for (int j = 0; j < alphabets.size; j++) {
			if (a_word[i] == alphabets.s[j]) {
				return FALSE;
			}
		}
		alphabets.s[i] = a_word[i];
		alphabets.size++;
	}
	return TRUE;
}

int main(void) {
	int answer_size = 0;
	int *binary_counter;
	char a_word[100000];
	a_word[0] = '\0';
	words subject;
	scanf("%d", &(subject.size));
	subject.s = (char *)malloc(sizeof(char)*subject.size);
	binary_counter = (int *)malloc(sizeof(int) * subject.size);
	for (int i = 0; i < subject.size; i++) {
		binary_counter[i] = 0;
	}
	binary_counter[subject.size] = '\0';
	scanf("%s", subject.s);
	for (int i = 1; i < pow(2, subject.size); i++) {
		plus_binary_counter(binary_counter, subject.size);
		for (int j = 0; j < subject.size; j++) {
			if (binary_counter[j] == 1) {
				sprintf(a_word, "%s%c", a_word, subject.s[j]);
			}
		}
		if (is_doubled(a_word) == TRUE) {
			answer_size++;
		}
		a_word[0] = '\0';
	}
	printf("%d\n", answer_size);
	return 0;
}