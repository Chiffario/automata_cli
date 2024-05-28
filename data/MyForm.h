#pragma once
#include <iostream>
#include <fstream>
#include <string.h>
#include <Windows.h>
namespace Project1 {

	using namespace System;
	using namespace System::ComponentModel;
	using namespace System::Collections;
	using namespace System::Windows::Forms;
	using namespace System::Data;
	using namespace System::Drawing;
	using namespace System::IO;

	/// <summary>
	/// Сводка для MyForm
	/// </summary>
	public ref class MyForm : public System::Windows::Forms::Form
	{
	public:
		MyForm(void)
		{
			InitializeComponent();
			//
			//TODO: добавьте код конструктора
			//
		}

	protected:
		/// <summary>
		/// Освободить все используемые ресурсы.
		/// </summary>
		~MyForm()
		{
			if (components)
			{
				delete components;
			}
		}
	private: System::Windows::Forms::TextBox^ textInput;
	private: System::Windows::Forms::TextBox^ textOutput;
	private: System::Windows::Forms::Button^ buttonProcceced;

	protected:

	protected:


	private: System::Windows::Forms::Label^ label1;
	private: System::Windows::Forms::Label^ label2;
	private: System::Windows::Forms::Button^ buttonInput;
	private: System::Windows::Forms::Button^ button1;



	private: System::Windows::Forms::Button^ button4;

	private: System::Windows::Forms::DataGridView^ dataGridView1;
	private: System::Windows::Forms::DataGridView^ dataGridView2;
	private: System::Windows::Forms::TextBox^ textBoxOfError;
	private: System::Windows::Forms::TextBox^ textBoxDesk;
	private: System::Windows::Forms::TextBox^ textBoxPsev;

	private: System::Windows::Forms::DataGridView^ dataGridView3;
	private: System::Windows::Forms::DataGridView^ dataGridView4;
	private: System::Windows::Forms::DataGridView^ dataGridView5;
	private: System::Windows::Forms::Label^ label3;
	private: System::Windows::Forms::Label^ label4;
	private: System::Windows::Forms::Label^ label5;
	private: System::Windows::Forms::Label^ label6;
	private: System::Windows::Forms::Label^ label7;
	private: System::Windows::Forms::OpenFileDialog^ openFileDialog1;
	private: System::Windows::Forms::Label^ label8;
	private: System::Windows::Forms::Label^ label9;
	private: System::Windows::Forms::Label^ label10;

	private:
		/// <summary>
		/// Обязательная переменная конструктора.
		/// </summary>
		System::ComponentModel::Container ^components;

#pragma region Windows Form Designer generated code
		/// <summary>
		/// Требуемый метод для поддержки конструктора — не изменяйте 
		/// содержимое этого метода с помощью редактора кода.
		/// </summary>
		void InitializeComponent(void)
		{
			this->textInput = (gcnew System::Windows::Forms::TextBox());
			this->textOutput = (gcnew System::Windows::Forms::TextBox());
			this->buttonProcceced = (gcnew System::Windows::Forms::Button());
			this->label1 = (gcnew System::Windows::Forms::Label());
			this->label2 = (gcnew System::Windows::Forms::Label());
			this->buttonInput = (gcnew System::Windows::Forms::Button());
			this->button1 = (gcnew System::Windows::Forms::Button());
			this->button4 = (gcnew System::Windows::Forms::Button());
			this->dataGridView1 = (gcnew System::Windows::Forms::DataGridView());
			this->dataGridView2 = (gcnew System::Windows::Forms::DataGridView());
			this->textBoxOfError = (gcnew System::Windows::Forms::TextBox());
			this->textBoxDesk = (gcnew System::Windows::Forms::TextBox());
			this->textBoxPsev = (gcnew System::Windows::Forms::TextBox());
			this->dataGridView3 = (gcnew System::Windows::Forms::DataGridView());
			this->dataGridView4 = (gcnew System::Windows::Forms::DataGridView());
			this->dataGridView5 = (gcnew System::Windows::Forms::DataGridView());
			this->label3 = (gcnew System::Windows::Forms::Label());
			this->label4 = (gcnew System::Windows::Forms::Label());
			this->label5 = (gcnew System::Windows::Forms::Label());
			this->label6 = (gcnew System::Windows::Forms::Label());
			this->label7 = (gcnew System::Windows::Forms::Label());
			this->openFileDialog1 = (gcnew System::Windows::Forms::OpenFileDialog());
			this->label8 = (gcnew System::Windows::Forms::Label());
			this->label9 = (gcnew System::Windows::Forms::Label());
			this->label10 = (gcnew System::Windows::Forms::Label());
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView1))->BeginInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView2))->BeginInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView3))->BeginInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView4))->BeginInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView5))->BeginInit();
			this->SuspendLayout();
			// 
			// textInput
			// 
			this->textInput->Location = System::Drawing::Point(31, 41);
			this->textInput->Margin = System::Windows::Forms::Padding(4);
			this->textInput->Multiline = true;
			this->textInput->Name = L"textInput";
			this->textInput->ScrollBars = System::Windows::Forms::ScrollBars::Both;
			this->textInput->Size = System::Drawing::Size(540, 486);
			this->textInput->TabIndex = 0;
			// 
			// textOutput
			// 
			this->textOutput->Location = System::Drawing::Point(579, 41);
			this->textOutput->Margin = System::Windows::Forms::Padding(4);
			this->textOutput->Multiline = true;
			this->textOutput->Name = L"textOutput";
			this->textOutput->ScrollBars = System::Windows::Forms::ScrollBars::Both;
			this->textOutput->Size = System::Drawing::Size(548, 486);
			this->textOutput->TabIndex = 1;
			// 
			// buttonProcceced
			// 
			this->buttonProcceced->Location = System::Drawing::Point(1745, 409);
			this->buttonProcceced->Margin = System::Windows::Forms::Padding(4);
			this->buttonProcceced->Name = L"buttonProcceced";
			this->buttonProcceced->Size = System::Drawing::Size(167, 58);
			this->buttonProcceced->TabIndex = 2;
			this->buttonProcceced->Text = L"Обработать";
			this->buttonProcceced->UseVisualStyleBackColor = true;
			this->buttonProcceced->Click += gcnew System::EventHandler(this, &MyForm::buttonProcceced_Click);
			// 
			// label1
			// 
			this->label1->AutoSize = true;
			this->label1->Location = System::Drawing::Point(205, 17);
			this->label1->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label1->Name = L"label1";
			this->label1->Size = System::Drawing::Size(105, 16);
			this->label1->TabIndex = 3;
			this->label1->Text = L"Входной_текст";
			// 
			// label2
			// 
			this->label2->AutoSize = true;
			this->label2->Location = System::Drawing::Point(745, 17);
			this->label2->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label2->Name = L"label2";
			this->label2->Size = System::Drawing::Size(114, 16);
			this->label2->TabIndex = 4;
			this->label2->Text = L"Выходной_текст";
			// 
			// buttonInput
			// 
			this->buttonInput->Location = System::Drawing::Point(1541, 409);
			this->buttonInput->Margin = System::Windows::Forms::Padding(4);
			this->buttonInput->Name = L"buttonInput";
			this->buttonInput->Size = System::Drawing::Size(196, 58);
			this->buttonInput->TabIndex = 5;
			this->buttonInput->Text = L"Ввод текста ";
			this->buttonInput->UseVisualStyleBackColor = true;
			this->buttonInput->Click += gcnew System::EventHandler(this, &MyForm::buttonInput_Click);
			// 
			// button1
			// 
			this->button1->Location = System::Drawing::Point(1847, 497);
			this->button1->Name = L"button1";
			this->button1->Size = System::Drawing::Size(65, 30);
			this->button1->TabIndex = 6;
			this->button1->Text = L"reset";
			this->button1->UseVisualStyleBackColor = true;
			this->button1->Click += gcnew System::EventHandler(this, &MyForm::button1_Click);
			// 
			// button4
			// 
			this->button4->Location = System::Drawing::Point(1541, 474);
			this->button4->Name = L"button4";
			this->button4->Size = System::Drawing::Size(196, 53);
			this->button4->TabIndex = 10;
			this->button4->Text = L"Распознать слова";
			this->button4->UseVisualStyleBackColor = true;
			this->button4->Click += gcnew System::EventHandler(this, &MyForm::button4_Click);
			// 
			// dataGridView1
			// 
			this->dataGridView1->AutoSizeColumnsMode = System::Windows::Forms::DataGridViewAutoSizeColumnsMode::Fill;
			this->dataGridView1->AutoSizeRowsMode = System::Windows::Forms::DataGridViewAutoSizeRowsMode::AllCells;
			this->dataGridView1->ColumnHeadersHeight = 18;
			this->dataGridView1->Location = System::Drawing::Point(31, 567);
			this->dataGridView1->Name = L"dataGridView1";
			this->dataGridView1->ReadOnly = true;
			this->dataGridView1->RowHeadersVisible = false;
			this->dataGridView1->RowHeadersWidth = 51;
			this->dataGridView1->RowTemplate->Height = 24;
			this->dataGridView1->ScrollBars = System::Windows::Forms::ScrollBars::Vertical;
			this->dataGridView1->SelectionMode = System::Windows::Forms::DataGridViewSelectionMode::CellSelect;
			this->dataGridView1->Size = System::Drawing::Size(366, 209);
			this->dataGridView1->TabIndex = 12;
			// 
			// dataGridView2
			// 
			this->dataGridView2->AutoSizeColumnsMode = System::Windows::Forms::DataGridViewAutoSizeColumnsMode::Fill;
			this->dataGridView2->AutoSizeRowsMode = System::Windows::Forms::DataGridViewAutoSizeRowsMode::AllCells;
			this->dataGridView2->ColumnHeadersHeight = 18;
			this->dataGridView2->Location = System::Drawing::Point(403, 567);
			this->dataGridView2->Name = L"dataGridView2";
			this->dataGridView2->ReadOnly = true;
			this->dataGridView2->RowHeadersVisible = false;
			this->dataGridView2->RowHeadersWidth = 51;
			this->dataGridView2->RowTemplate->Height = 24;
			this->dataGridView2->ScrollBars = System::Windows::Forms::ScrollBars::Vertical;
			this->dataGridView2->SelectionMode = System::Windows::Forms::DataGridViewSelectionMode::CellSelect;
			this->dataGridView2->Size = System::Drawing::Size(366, 209);
			this->dataGridView2->TabIndex = 13;
			// 
			// textBoxOfError
			// 
			this->textBoxOfError->Location = System::Drawing::Point(1150, 424);
			this->textBoxOfError->Margin = System::Windows::Forms::Padding(4);
			this->textBoxOfError->Multiline = true;
			this->textBoxOfError->Name = L"textBoxOfError";
			this->textBoxOfError->ReadOnly = true;
			this->textBoxOfError->ScrollBars = System::Windows::Forms::ScrollBars::Both;
			this->textBoxOfError->Size = System::Drawing::Size(376, 103);
			this->textBoxOfError->TabIndex = 14;
			// 
			// textBoxDesk
			// 
			this->textBoxDesk->Location = System::Drawing::Point(1150, 41);
			this->textBoxDesk->Multiline = true;
			this->textBoxDesk->Name = L"textBoxDesk";
			this->textBoxDesk->ReadOnly = true;
			this->textBoxDesk->ScrollBars = System::Windows::Forms::ScrollBars::Both;
			this->textBoxDesk->Size = System::Drawing::Size(376, 360);
			this->textBoxDesk->TabIndex = 15;
			// 
			// textBoxPsev
			// 
			this->textBoxPsev->Location = System::Drawing::Point(1541, 41);
			this->textBoxPsev->Multiline = true;
			this->textBoxPsev->Name = L"textBoxPsev";
			this->textBoxPsev->ReadOnly = true;
			this->textBoxPsev->ScrollBars = System::Windows::Forms::ScrollBars::Both;
			this->textBoxPsev->Size = System::Drawing::Size(371, 360);
			this->textBoxPsev->TabIndex = 16;
			// 
			// dataGridView3
			// 
			this->dataGridView3->AutoSizeColumnsMode = System::Windows::Forms::DataGridViewAutoSizeColumnsMode::Fill;
			this->dataGridView3->AutoSizeRowsMode = System::Windows::Forms::DataGridViewAutoSizeRowsMode::AllCells;
			this->dataGridView3->ColumnHeadersHeight = 18;
			this->dataGridView3->Location = System::Drawing::Point(775, 567);
			this->dataGridView3->Name = L"dataGridView3";
			this->dataGridView3->ReadOnly = true;
			this->dataGridView3->RowHeadersVisible = false;
			this->dataGridView3->RowHeadersWidth = 51;
			this->dataGridView3->RowTemplate->Height = 24;
			this->dataGridView3->ScrollBars = System::Windows::Forms::ScrollBars::Vertical;
			this->dataGridView3->SelectionMode = System::Windows::Forms::DataGridViewSelectionMode::CellSelect;
			this->dataGridView3->Size = System::Drawing::Size(366, 209);
			this->dataGridView3->TabIndex = 18;
			// 
			// dataGridView4
			// 
			this->dataGridView4->AutoSizeColumnsMode = System::Windows::Forms::DataGridViewAutoSizeColumnsMode::Fill;
			this->dataGridView4->AutoSizeRowsMode = System::Windows::Forms::DataGridViewAutoSizeRowsMode::AllCells;
			this->dataGridView4->ColumnHeadersHeight = 18;
			this->dataGridView4->Location = System::Drawing::Point(1150, 567);
			this->dataGridView4->Name = L"dataGridView4";
			this->dataGridView4->ReadOnly = true;
			this->dataGridView4->RowHeadersVisible = false;
			this->dataGridView4->RowHeadersWidth = 51;
			this->dataGridView4->RowTemplate->Height = 24;
			this->dataGridView4->ScrollBars = System::Windows::Forms::ScrollBars::Vertical;
			this->dataGridView4->SelectionMode = System::Windows::Forms::DataGridViewSelectionMode::CellSelect;
			this->dataGridView4->Size = System::Drawing::Size(366, 209);
			this->dataGridView4->TabIndex = 19;
			// 
			// dataGridView5
			// 
			this->dataGridView5->AutoSizeColumnsMode = System::Windows::Forms::DataGridViewAutoSizeColumnsMode::Fill;
			this->dataGridView5->AutoSizeRowsMode = System::Windows::Forms::DataGridViewAutoSizeRowsMode::AllCells;
			this->dataGridView5->ColumnHeadersHeight = 18;
			this->dataGridView5->Location = System::Drawing::Point(1528, 567);
			this->dataGridView5->Name = L"dataGridView5";
			this->dataGridView5->ReadOnly = true;
			this->dataGridView5->RowHeadersVisible = false;
			this->dataGridView5->RowHeadersWidth = 51;
			this->dataGridView5->RowTemplate->Height = 24;
			this->dataGridView5->ScrollBars = System::Windows::Forms::ScrollBars::Vertical;
			this->dataGridView5->SelectionMode = System::Windows::Forms::DataGridViewSelectionMode::CellSelect;
			this->dataGridView5->Size = System::Drawing::Size(366, 209);
			this->dataGridView5->TabIndex = 20;
			// 
			// label3
			// 
			this->label3->AutoSize = true;
			this->label3->Location = System::Drawing::Point(1267, 17);
			this->label3->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label3->Name = L"label3";
			this->label3->Size = System::Drawing::Size(136, 16);
			this->label3->TabIndex = 21;
			this->label3->Text = L"Дискрипторный код";
			// 
			// label4
			// 
			this->label4->AutoSize = true;
			this->label4->Location = System::Drawing::Point(1706, 17);
			this->label4->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label4->Name = L"label4";
			this->label4->Size = System::Drawing::Size(79, 16);
			this->label4->TabIndex = 22;
			this->label4->Text = L"Псевдокод";
			// 
			// label5
			// 
			this->label5->AutoSize = true;
			this->label5->Location = System::Drawing::Point(1314, 404);
			this->label5->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label5->Name = L"label5";
			this->label5->Size = System::Drawing::Size(57, 16);
			this->label5->TabIndex = 23;
			this->label5->Text = L"Ошибки";
			// 
			// label6
			// 
			this->label6->AutoSize = true;
			this->label6->Location = System::Drawing::Point(169, 548);
			this->label6->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label6->Name = L"label6";
			this->label6->Size = System::Drawing::Size(122, 16);
			this->label6->TabIndex = 24;
			this->label6->Text = L"Идентификаторы";
			// 
			// label7
			// 
			this->label7->AutoSize = true;
			this->label7->Location = System::Drawing::Point(559, 548);
			this->label7->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label7->Name = L"label7";
			this->label7->Size = System::Drawing::Size(77, 16);
			this->label7->TabIndex = 25;
			this->label7->Text = L"Константы";
			// 
			// openFileDialog1
			// 
			this->openFileDialog1->FileName = L"openFileDialog1";
			// 
			// label8
			// 
			this->label8->AutoSize = true;
			this->label8->Location = System::Drawing::Point(929, 548);
			this->label8->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label8->Name = L"label8";
			this->label8->Size = System::Drawing::Size(116, 16);
			this->label8->TabIndex = 26;
			this->label8->Text = L"Ключевые слова";
			// 
			// label9
			// 
			this->label9->AutoSize = true;
			this->label9->Location = System::Drawing::Point(1267, 548);
			this->label9->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label9->Name = L"label9";
			this->label9->Size = System::Drawing::Size(156, 16);
			this->label9->TabIndex = 27;
			this->label9->Text = L"Операторы отношений";
			// 
			// label10
			// 
			this->label10->AutoSize = true;
			this->label10->Location = System::Drawing::Point(1706, 548);
			this->label10->Margin = System::Windows::Forms::Padding(4, 0, 4, 0);
			this->label10->Name = L"label10";
			this->label10->Size = System::Drawing::Size(47, 16);
			this->label10->TabIndex = 28;
			this->label10->Text = L"Знаки";
			// 
			// MyForm
			// 
			this->AutoScaleDimensions = System::Drawing::SizeF(8, 16);
			this->AutoScaleMode = System::Windows::Forms::AutoScaleMode::Font;
			this->AutoSize = true;
			this->AutoSizeMode = System::Windows::Forms::AutoSizeMode::GrowAndShrink;
			this->ClientSize = System::Drawing::Size(1924, 788);
			this->Controls->Add(this->label10);
			this->Controls->Add(this->label9);
			this->Controls->Add(this->label8);
			this->Controls->Add(this->label7);
			this->Controls->Add(this->label6);
			this->Controls->Add(this->label5);
			this->Controls->Add(this->label4);
			this->Controls->Add(this->label3);
			this->Controls->Add(this->dataGridView5);
			this->Controls->Add(this->dataGridView4);
			this->Controls->Add(this->dataGridView3);
			this->Controls->Add(this->textBoxPsev);
			this->Controls->Add(this->textBoxDesk);
			this->Controls->Add(this->textBoxOfError);
			this->Controls->Add(this->dataGridView2);
			this->Controls->Add(this->dataGridView1);
			this->Controls->Add(this->button4);
			this->Controls->Add(this->button1);
			this->Controls->Add(this->buttonInput);
			this->Controls->Add(this->label2);
			this->Controls->Add(this->label1);
			this->Controls->Add(this->buttonProcceced);
			this->Controls->Add(this->textOutput);
			this->Controls->Add(this->textInput);
			this->Margin = System::Windows::Forms::Padding(4);
			this->Name = L"MyForm";
			this->Text = L"MyForm";
			this->WindowState = System::Windows::Forms::FormWindowState::Maximized;
			this->Activated += gcnew System::EventHandler(this, &MyForm::MyForm_Activated);
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView1))->EndInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView2))->EndInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView3))->EndInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView4))->EndInit();
			(cli::safe_cast<System::ComponentModel::ISupportInitialize^>(this->dataGridView5))->EndInit();
			this->ResumeLayout(false);
			this->PerformLayout();

		}
#pragma endregion

	private: System::Void buttonInput_Click(System::Object^ sender, System::EventArgs^ e) {
		SetConsoleCP(1251);
		SetConsoleOutputCP(1251);
		String^ fileName = "";
		if (openFileDialog1->ShowDialog() == Windows::Forms::DialogResult::OK) {
			fileName = openFileDialog1->FileName;
		}
		try {
			StreamReader^ file = File::OpenText(fileName);
			textInput->Text = file->ReadToEnd();
		}
		catch (Exception^ e) {
			MessageBox::Show(this, "Файл не был открыт", "Ошибка", MessageBoxButtons::OK, MessageBoxIcon::Error);
		}
	}

int amount = 1;

private: System::Void buttonProcceced_Click(System::Object^ sender, System::EventArgs^ e) {
	textOutput->Text = "";
	amount = 1;
	int s = 0;
	int x=0;
	/*
	0 = _
	1 = t
	2 = /
	3 = *
	4 = A	
	*/
	String^ memoryOfComment = "";
	StringReader^ reader = gcnew StringReader(textInput->Text);
	String^ a;
	while ((a = reader->ReadLine()) != nullptr)
	{
		String^ b = "";
		int n = a->Length;
		for (int i = 0; i < n; i++) {
			int n1 = b->Length;
			if (a[i] == ' ') x = 0;
			else if (a[i] == '	')x = 1;
			else if (a[i] == '/') x = 2;
			else if (a[i] == '*') x = 3;
			else x = 4;
			switch (s) {
			case 0:
				switch (x) {
				case 0:
					s = 2;
					break;
				case 1:
					s = 2;
					break;
				case 2:
					s = 3;
					b += a[i];
					break;
				case 3:
					s = 1;
					b += a[i];
					break;
				case 4:
					s = 1;
					b += a[i];
					break;
				}break;
			case 1:
				switch (x) {
				case 0:
					s = 4;
					b += a[i];
					break;
				case 1:
					s = 2;
					break;
				case 2:
					s = 3;
					b += a[i];
					break;
				case 3:
					s = 1;
					b += a[i];
					break;
				case 4:
					s = 1;
					b += a[i];
					break;
				}break;
			case 2:
				switch (x) {
				case 0:
					s = 2;
					break;
				case 1:
					s = 2;
					break;
				case 2:
					s = 3;
					b += a[i];
					break;
				case 3:
					s = 1;
					b += a[i];
					break;
				case 4:
					s = 1;
					b += a[i];
					break;
				}break;
			case 3:
				switch (x) {
				case 0:
					s = 4;
					b += a[i];
					break;
				case 1:
					s = 2;
					break;
				case 2:
					s = 6;
					b = b->Remove(n1 - 1);
					break;
				case 3:
					s = 5;
					b = b->Remove(n1 - 1);
					break;
				case 4:
					s = 1;
					b += a[i];
					break;
				}break;
			case 4:
				switch (x) {
				case 0:
					s = 2;
					break;
				case 1:
					s = 2;
					break;
				case 2:
					s = 3;
					b += a[i];
					break;
				case 3:
					s = 1;
					b += a[i];
					break;
				case 4:
					s = 1;
					b += a[i];
					break;
				}break;
			case 5:
				switch (x) {
				case 0:
					s = 5;
					break;
				case 1:
					s = 5;
					break;
				case 2:
					s = 5;
					break;
				case 3:
					s = 7;
					break;
				case 4:
					s = 5;
					break;
				}break;
			case 6:
				switch (x) {
				case 0:
					s = 6;
					break;
				case 1:
					s = 6;
					break;
				case 2:
					s = 6;
					break;
				case 3:
					s = 6;
					break;
				case 4:
					s = 6;
					break;
				}break;
			case 7:
				switch (x) {
				case 0:
					s = 5;
					break;
				case 1:
					s = 5;
					break;
				case 2:
					s = 8;
					break;
				case 3:
					s = 5;
					break;
				case 4:
					s = 5;
					break;
				}break;
			case 8:
				switch (x) {
				case 0:
					s = 2;
					break;
				case 1:
					s = 2;
					break;
				case 2:
					s = 3;
					b += a[i];
					break;
				case 3:
					s = 1;
					b += a[i];
					break;
				case 4:
					s = 1;
					b += a[i];
					break;
				}break;
			}
			//textOutput->Text += s.ToString();
		}
		if (b != "") {
			textOutput->Text += amount + ") ";
			amount++;
			textOutput->Text += b;
			textOutput->Text += Environment::NewLine;
		}
		if (s == 5 || s == 7)s = 5;
		else s = 0;
	}
}
private: System::Void button1_Click(System::Object^ sender, System::EventArgs^ e) {
	amount = 1;

	amountConst = amountID = amountError = amountWord = amountCh = amountOp = 1;
	textOutput->Text = "";
	dataGridView1->Columns->Clear();
	dataGridView1->ColumnCount = 3;
	dataGridView1->Columns[0]->Name = "10";
	dataGridView1->Columns[1]->Name = "Иден";
	dataGridView1->Columns[2]->Name = "п/к";

	dataGridView2->Columns->Clear();
	dataGridView2->ColumnCount = 3;
	dataGridView2->Columns[0]->Name = "20";
	dataGridView2->Columns[1]->Name = "Конст";
	dataGridView2->Columns[2]->Name = "п/к";

	dataGridView3->Columns->Clear();
	dataGridView3->ColumnCount = 3;
	dataGridView3->Columns[0]->Name = "30";
	dataGridView3->Columns[1]->Name = "Ключевые слова";
	dataGridView3->Columns[2]->Name = "код";

	dataGridView4->Columns->Clear();
	dataGridView4->ColumnCount = 3;
	dataGridView4->Columns[0]->Name = "40";
	dataGridView4->Columns[1]->Name = "Операторы отношений";
	dataGridView4->Columns[2]->Name = "код";

	dataGridView5->Columns->Clear();
	dataGridView5->ColumnCount = 3;
	dataGridView5->Columns[0]->Name = "50";
	dataGridView5->Columns[1]->Name = "Знаки";
	dataGridView5->Columns[2]->Name = "код";

	textBoxDesk->Text = "";
	textBoxPsev->Text = "";
	textBoxOfError->Text = "";
}

	   array<String^>^ arrID = gcnew array<String^>(3);
	   array<String^>^ arrConst = gcnew array<String^>(3);
	   array<String^>^ arrWord = gcnew array<String^>(3);
	   array<String^>^ arrOp = gcnew array<String^>(3);
	   array<String^>^ arrCh = gcnew array<String^>(3);
	   int amountID = 1;
	   int amountConst = 1;
	   int amountWord = 1;
	   int amountOp = 1;
	   int amountCh = 1;
	   int amountError = 1;

private: System::Void button4_Click(System::Object^ sender, System::EventArgs^ e) {
	int s = 0;
	int x = 0;
	bool fl = 0;
	bool flError = 0;

	StringReader^ reader = gcnew StringReader(textOutput->Text);
	String^ a;
	while ((a = reader->ReadLine()) != nullptr)
	{
		String^ b = "";
		int number = 0;
		int n = a->Length;
		while (a[number] != ')')
			number++;
		for (int i = number+1; i < n; i++) {
			//textBoxOfError->Text += Convert::ToString(s) + " ";
			switch (s) {
			case 0:
				switch (a[i]) {
				case 'a':
					s = 1;
					b += a[i];
					break;
				case 'b':
					s = 7;
					b += a[i];
					break;
				case 'c':
					s = 22;
					b += a[i];
					break;
				case 'd':
					s = 47;
					b += a[i];
					break;
				case 'e':
					s = 63;
					b += a[i];
					break;
				case 'f':
					s = 79;
					b += a[i];
					break;
				case'g':
					s = 95;
					b += a[i];
					break;
				case 'i':
					s = 99;
					b += a[i];
					break;
				case'l':
					s = 107;
					b += a[i];
					break;
				case'm':
					s = 111;
					b += a[i];
					break;
				case 'n':
					s = 118;
					b += a[i];
					break;
				case'o':
					s = 137;
					b += a[i];
					break;
				case'p':
					s = 146;
					b += a[i];
					break;
				case'r':
					s = 165;
					b += a[i];
					break;
				case's':
					s = 171;
					b += a[i];
					break;
				case't':
					s = 199;
					b += a[i];
					break;
				case'u':
					s = 217;
					b += a[i];
					break;
				case'v':
					s = 232;
					b += a[i];
					break;
				case'w':
					s = 242;
					b += a[i];
					break;
				case';':
					s = 247;
					b += a[i];
					break;
				case'(':
					s = 248;
					b += a[i];
					break;
				case')':
					s = 249;
					b += a[i];
					break;
				case'[':
					s = 250;
					b += a[i];
					break;
				case']':
					s = 251;
					b += a[i];
					break;
				case'-':
					s = 252;
					b += a[i];
					break;
				case'+':
					s = 256;
					b += a[i];
					break;
				case':':
					s = 259;
					b += a[i];
					break;
				case'.':
					s = 261;
					b += a[i];
					break;
				case'!':
					s = 262;
					b += a[i];
					break;
				case'*':
					s = 264;
					b += a[i];
					break;
				case'&':
					s = 266;
					b += a[i];
					break;
				case'/':
					s = 268;
					b += a[i];
					break;
				case'%':
					s = 270;
					b += a[i];
					break;
				case'<':
					s = 271;
					b += a[i];
					break;
				case'>':
					s = 274;
					b += a[i];
					break;
				case'=':
					s = 277;
					b += a[i];
					break;
				case'^':
					s = 279;
					b += a[i];
					break;
				case'|':
					s = 280;
					b += a[i];
					break;
				case',':
					s = 282;
					b += a[i];
					break;
				case'{':
					s = 283;
					b += a[i];
					break;
				case'}':
					s = 284;
					b += a[i];
					break;
				case'"':
					s = 285;
					b += a[i];
					break;
				case'\'':
					s = 288;
					b += a[i];
					break;
				case'0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 292;
					b += a[i];
					break;
				case'_':
					s = 303;
					b += a[i];
					break;
				case' ':
					s = 0;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122) {
						s = 291;
						b += a[i];
					}
					else s = 0;
					break;
				}
				break;
			case 1:
				switch (a[i]) {
				case'u':
					s = 4;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 2:
				switch (a[i]) {
				case'm':
					s = 3;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 3:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 4:
				switch (a[i]) {
				case't':
					s = 5;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 5:
				switch (a[i]) {
				case'o':
					s = 6;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 6:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 7:
				switch (a[i]) {
				case'i':
					s = 8;
					b += a[i];
					break;
				case'o':
					s = 15;
					b += a[i];
					break;
				case'r':
					s = 18;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 8:
				switch (a[i]) {
				case't':
					s = 9;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 9:
				switch (a[i]) {
				case'a':
					s = 10;
					b += a[i];
					break;
				case'o':
					s = 13;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 10:
				switch (a[i]) {
				case'n':
					s = 11;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 11:
				switch (a[i]) {
				case'd':
					s = 12;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 12:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 13:
				switch (a[i]) {
				case'r':
					s = 14;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 14:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 15:
				switch (a[i]) {
				case'o':
					s = 16;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 16:
				switch (a[i]) {
				case'l':
					s = 17;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 17:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 18:
				switch (a[i]) {
				case'e':
					s = 19;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 19:
				switch (a[i]) {
				case'a':
					s = 20;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 20:
				switch (a[i]) {
				case'k':
					s = 21;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 21:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 22:
				switch (a[i]) {
				case'a':
					s = 23;
					b += a[i];
					break;
				case'h':
					s = 29;
					b += a[i];
					break;
				case'l':
					s = 32;
					b += a[i];
					break;
				case'o':
					s = 36;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 23:
				switch (a[i]) {
				case's':
					s = 24;
					b += a[i];
					break;
				case't':
					s = 26;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 24:
				switch (a[i]) {
				case'e':
					s = 25;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 25:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 26:
				switch (a[i]) {
				case'c':
					s = 27;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 27:
				switch (a[i]) {
				case'h':
					s = 28;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;

			case 28:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 29:
				switch (a[i]) {
				case'a':
					s = 30;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 30:
				switch (a[i]) {
				case'r':
					s = 31;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 31:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 32:
				switch (a[i]) {
				case'a':
					s = 33;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 33:
				switch (a[i]) {
				case's':
					s = 34;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 34:
				switch (a[i]) {
				case's':
					s = 35;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 35:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 36:
				switch (a[i]) {
				case'm':
					s = 37;
					b += a[i];
					break;
				case'n':
					s = 40;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 37:
				switch (a[i]) {
				case'p':
					s = 38;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 38:
				switch (a[i]) {
				case'l':
					s = 39;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 39:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 40:
				switch (a[i]) {
				case'c':
					s = 41;
					b += a[i];
					break;
				case's':
					s = 45;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 41:
				switch (a[i]) {
				case'e':
					s = 42;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 42:
				switch (a[i]) {
				case'p':
					s = 43;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 43:
				switch (a[i]) {
				case't':
					s = 44;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 44:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 45:
				switch (a[i]) {
				case't':
					s = 46;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 46:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 47:
				switch (a[i]) {
				case'e':
					s = 48;
					b += a[i];
					break;
				case'o':
					s = 58;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 48:
				switch (a[i]) {
				case'f':
					s = 49;
					b += a[i];
					break;
				case'l':
					s = 54;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 49:
				switch (a[i]) {
				case'a':
					s = 50;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 50:
				switch (a[i]) {
				case'u':
					s = 51;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 51:
				switch (a[i]) {
				case'l':
					s = 52;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 52:
				switch (a[i]) {
				case't':
					s = 53;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 53:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 54:
				switch (a[i]) {
				case'e':
					s = 55;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 55:
				switch (a[i]) {
				case't':
					s = 56;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 56:
				switch (a[i]) {
				case'e':
					s = 57;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 57:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 58:
				switch (a[i]) {
				case'u':
					s = 59;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 59:
				switch (a[i]) {
				case'b':
					s = 60;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 60:
				switch (a[i]) {
				case'l':
					s = 61;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 61:
				switch (a[i]) {
				case'e':
					s = 62;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 62:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 63:
				switch (a[i]) {
				case'l':
					s = 64;
					b += a[i];
					break;
				case'n':
					s = 67;
					b += a[i];
					break;
				case'x':
					s = 70;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 64:
				switch (a[i]) {
				case's':
					s = 65;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 65:
				switch (a[i]) {
				case'e':
					s = 66;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 66:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 67:
				switch (a[i]) {
				case'u':
					s = 68;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 68:
				switch (a[i]) {
				case'm':
					s = 69;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 69:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 70:
				switch (a[i]) {
				case'p':
					s = 71;
					b += a[i];
					break;
				case't':
					s = 75;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 71:
				switch (a[i]) {
				case'o':
					s = 72;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 72:
				switch (a[i]) {
				case'r':
					s = 73;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 73:
				switch (a[i]) {
				case't':
					s = 74;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 74:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 75:
				switch (a[i]) {
				case'e':
					s = 76;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 76:
				switch (a[i]) {
				case'r':
					s = 77;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 77:
				switch (a[i]) {
				case'n':
					s = 78;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 78:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 79:
				switch (a[i]) {
				case'a':
					s = 80;
					b += a[i];
					break;
				case'l':
					s = 84;
					b += a[i];
					break;
				case'o':
					s = 88;
					b += a[i];
					break;
				case'r':
					s = 90;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 80:
				switch (a[i]) {
				case'l':
					s = 81;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 81:
				switch (a[i]) {
				case's':
					s = 82;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 82:
				switch (a[i]) {
				case'e':
					s = 83;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 83:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 84:
				switch (a[i]) {
				case'o':
					s = 85;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 85:
				switch (a[i]) {
				case'a':
					s = 86;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 86:
				switch (a[i]) {
				case't':
					s = 87;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 87:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 88:
				switch (a[i]) {
				case'r':
					s = 89;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 89:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 90:
				switch (a[i]) {
				case'i':
					s = 91;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 91:
				switch (a[i]) {
				case'e':
					s = 92;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 92:
				switch (a[i]) {
				case'n':
					s = 93;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 93:
				switch (a[i]) {
				case'd':
					s = 94;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 94:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 95:
				switch (a[i]) {
				case'o':
					s = 96;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 96:
				switch (a[i]) {
				case't':
					s = 97;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 97:
				switch (a[i]) {
				case'o':
					s = 98;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 98:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 99:
				switch (a[i]) {
				case'f':
					s = 100;
					b += a[i];
					break;
				case'n':
					s = 101;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 100:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 101:
				switch (a[i]) {
				case'l':
					s = 102;
					b += a[i];
					break;
				case't':
					s = 106;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 102:
				switch (a[i]) {
				case'i':
					s = 103;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 103:
				switch (a[i]) {
				case'n':
					s = 104;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 104:
				switch (a[i]) {
				case'e':
					s = 105;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 105:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 106:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 107:
				switch (a[i]) {
				case'o':
					s = 108;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 108:
				switch (a[i]) {
				case'n':
					s = 109;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 109:
				switch (a[i]) {
				case'g':
					s = 110;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 110:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 111:
				switch (a[i]) {
				case'u':
					s = 112;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 112:
				switch (a[i]) {
				case't':
					s = 113;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 113:
				switch (a[i]) {
				case'a':
					s = 114;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 114:
				switch (a[i]) {
				case'b':
					s = 115;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 115:
				switch (a[i]) {
				case'l':
					s = 116;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 116:
				switch (a[i]) {
				case'e':
					s = 117;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 117:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 118:
				switch (a[i]) {
				case'a':
					s = 119;
					b += a[i];
					break;
				case'e':
					s = 127;
					b += a[i];
					break;
				case'o':
					s = 129;
					b += a[i];
					break;
				case'u':
					s = 131;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 119:
				switch (a[i]) {
				case'm':
					s = 120;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 120:
				switch (a[i]) {
				case'e':
					s = 121;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 121:
				switch (a[i]) {
				case's':
					s = 122;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 122:
				switch (a[i]) {
				case'p':
					s = 123;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 123:
				switch (a[i]) {
				case'a':
					s = 124;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 124:
				switch (a[i]) {
				case'c':
					s = 125;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 125:
				switch (a[i]) {
				case'e':
					s = 126;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 126:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 127:
				switch (a[i]) {
				case'w':
					s = 128;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 128:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 129:
				switch (a[i]) {
				case't':
					s = 130;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 130:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 131:
				switch (a[i]) {
				case'l':
					s = 132;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 132:
				switch (a[i]) {
				case'l':
					s = 133;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 133:
				switch (a[i]) {
				case'p':
					s = 134;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 134:
				switch (a[i]) {
				case't':
					s = 135;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 135:
				switch (a[i]) {
				case'r':
					s = 136;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 136:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 137:
				switch (a[i]) {
				case'p':
					s = 138;
					b += a[i];
					break;
				case'r':
					s = 145;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 138:
				switch (a[i]) {
				case'e':
					s = 139;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 139:
				switch (a[i]) {
				case'r':
					s = 140;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 140:
				switch (a[i]) {
				case'a':
					s = 141;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 141:
				switch (a[i]) {
				case't':
					s = 142;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 142:
				switch (a[i]) {
				case'o':
					s = 143;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 143:
				switch (a[i]) {
				case'r':
					s = 144;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 144:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 145:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 146:
				switch (a[i]) {
				case'r':
					s = 147;
					b += a[i];
					break;
				case'u':
					s = 160;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 147:
				switch (a[i]) {
				case'i':
					s = 148;
					b += a[i];
					break;
				case'o':
					s = 153;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 148:
				switch (a[i]) {
				case'v':
					s = 149;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 149:
				switch (a[i]) {
				case'a':
					s = 150;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 150:
				switch (a[i]) {
				case't':
					s = 151;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 151:
				switch (a[i]) {
				case'e':
					s = 152;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 152:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 153:
				switch (a[i]) {
				case't':
					s = 154;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 154:
				switch (a[i]) {
				case'e':
					s = 155;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 155:
				switch (a[i]) {
				case'c':
					s = 156;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 156:
				switch (a[i]) {
				case't':
					s = 157;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 157:
				switch (a[i]) {
				case'e':
					s = 158;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 158:
				switch (a[i]) {
				case'd':
					s = 159;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 159:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 160:
				switch (a[i]) {
				case'b':
					s = 161;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 161:
				switch (a[i]) {
				case'l':
					s = 162;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 162:
				switch (a[i]) {
				case'i':
					s = 163;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 163:
				switch (a[i]) {
				case'c':
					s = 164;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 164:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 165:
				switch (a[i]) {
				case'e':
					s = 166;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 166:
				switch (a[i]) {
				case't':
					s = 167;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 167:
				switch (a[i]) {
				case'u':
					s = 168;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 168:
				switch (a[i]) {
				case'r':
					s = 169;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 169:
				switch (a[i]) {
				case'n':
					s = 170;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 170:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 171:
				switch (a[i]) {
				case'h':
					s = 172;
					b += a[i];
					break;
				case'i':
					s = 176;
					b += a[i];
					break;
				case't':
					s = 185;
					b += a[i];
					break;
				case'w':
					s = 194;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 172:
				switch (a[i]) {
				case'o':
					s = 173;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 173:
				switch (a[i]) {
				case'r':
					s = 174;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 174:
				switch (a[i]) {
				case't':
					s = 175;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 175:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 176:
				switch (a[i]) {
				case'g':
					s = 177;
					b += a[i];
					break;
				case'z':
					s = 181;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 177:
				switch (a[i]) {
				case'n':
					s = 178;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 178:
				switch (a[i]) {
				case'e':
					s = 179;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 179:
				switch (a[i]) {
				case'd':
					s = 180;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 180:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 181:
				switch (a[i]) {
				case'e':
					s = 182;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 182:
				switch (a[i]) {
				case'o':
					s = 183;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 183:
				switch (a[i]) {
				case'f':
					s = 184;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 184:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 185:
				switch (a[i]) {
				case'a':
					s = 186;
					b += a[i];
					break;
				case'r':
					s = 190;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 186:
				switch (a[i]) {
				case't':
					s = 187;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 187:
				switch (a[i]) {
				case'i':
					s = 188;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 188:
				switch (a[i]) {
				case'c':
					s = 189;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 189:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 190:
				switch (a[i]) {
				case'u':
					s = 191;
					b += a[i];
					break;
				case'i':
					s = 300;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 191:
				switch (a[i]) {
				case'c':
					s = 192;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 192:
				switch (a[i]) {
				case't':
					s = 193;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 193:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 194:
				switch (a[i]) {
				case'i':
					s = 195;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 195:
				switch (a[i]) {
				case't':
					s = 196;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 196:
				switch (a[i]) {
				case'c':
					s = 197;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 197:
				switch (a[i]) {
				case'h':
					s = 198;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 198:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 199:
				switch (a[i]) {
				case'e':
					s = 200;
					b += a[i];
					break;
				case'h':
					s = 207;
					b += a[i];
					break;
				case'r':
					s = 213;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 200:
				switch (a[i]) {
				case'm':
					s = 201;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 201:
				switch (a[i]) {
				case'p':
					s = 202;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 202:
				switch (a[i]) {
				case'l':
					s = 203;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 203:
				switch (a[i]) {
				case'a':
					s = 204;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 204:
				switch (a[i]) {
				case't':
					s = 205;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 205:
				switch (a[i]) {
				case'e':
					s = 206;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 206:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 207:
				switch (a[i]) {
				case'i':
					s = 208;
					b += a[i];
					break;
				case'r':
					s = 210;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 208:
				switch (a[i]) {
				case's':
					s = 209;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 209:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 210:
				switch (a[i]) {
				case'o':
					s = 211;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 211:
				switch (a[i]) {
				case'w':
					s = 212;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 212:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 213:
				switch (a[i]) {
				case'u':
					s = 214;
					b += a[i];
					break;
				case'y':
					s = 216;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 214:
				switch (a[i]) {
				case'e':
					s = 215;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 215:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 216:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 217:
				switch (a[i]) {
				case'n':
					s = 218;
					b += a[i];
					break;
				case's':
					s = 228;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 218:
				switch (a[i]) {
				case'i':
					s = 219;
					b += a[i];
					break;
				case's':
					s = 222;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 219:
				switch (a[i]) {
				case'o':
					s = 220;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 220:
				switch (a[i]) {
				case'n':
					s = 221;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 221:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 222:
				switch (a[i]) {
				case'i':
					s = 223;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 223:
				switch (a[i]) {
				case'g':
					s = 224;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 224:
				switch (a[i]) {
				case'n':
					s = 225;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 225:
				switch (a[i]) {
				case'e':
					s = 226;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 226:
				switch (a[i]) {
				case'd':
					s = 227;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 227:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 228:
				switch (a[i]) {
				case'i':
					s = 229;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 229:
				switch (a[i]) {
				case'n':
					s = 230;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 230:
				switch (a[i]) {
				case'g':
					s = 231;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 231:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 232:
				switch (a[i]) {
				case'i':
					s = 233;
					b += a[i];
					break;
				case'o':
					s = 239;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 233:
				switch (a[i]) {
				case'r':
					s = 234;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 234:
				switch (a[i]) {
				case't':
					s = 235;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 235:
				switch (a[i]) {
				case'u':
					s = 236;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 236:
				switch (a[i]) {
				case'a':
					s = 237;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 237:
				switch (a[i]) {
				case'l':
					s = 238;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 238:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 239:
				switch (a[i]) {
				case'i':
					s = 240;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 240:
				switch (a[i]) {
				case'd':
					s = 241;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 241:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 242:
				switch (a[i]) {
				case'h':
					s = 243;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 243:
				switch (a[i]) {
				case'i':
					s = 244;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 244:
				switch (a[i]) {
				case'l':
					s = 245;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 245:
				switch (a[i]) {
				case'e':
					s = 246;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 246:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 247:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 248:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 249:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 250:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 251:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 252:
				switch (a[i]) {
				case'>':
					s = 253;
					b += a[i];
					break;
				case'-':
					s = 254;
					b += a[i];
					break;
				case'=':
					s = 255;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 253:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 254:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 255:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 256:
				switch (a[i]) {
				case'+':
					s = 257;
					b += a[i];
					break;
				case'=':
					s = 258;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 257:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 258:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 259:
				switch (a[i]) {
				case':':
					s = 260;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 260:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 261:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 262:
				switch (a[i]) {
				case'=':
					s = 263;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 263:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 264:
				switch (a[i]) {
				case'=':
					s = 265;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 265:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 266:
				switch (a[i]) {
				case'&':
					s = 267;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 267:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 268:
				switch (a[i]) {
				case'=':
					s = 269;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 269:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 270:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 271:
				switch (a[i]) {
				case'<':
					s = 272;
					b += a[i];
					break;
				case'=':
					s = 273;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 272:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 273:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 274:
				switch (a[i]) {
				case'>':
					s = 275;
					b += a[i];
					break;
				case'=':
					s = 276;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 275:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 276:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 277:
				switch (a[i]) {
				case'=':
					s = 278;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 278:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 279:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 280:
				switch (a[i]) {
				case'|':
					s = 281;
					b += a[i];
					break;
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 281:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 282:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 283:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 284:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 285:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 286:
				switch (a[i]) {
				case'\"':
					fl = 1;
					i--;
					break;
				default:
					b += a[i];
					s = 286;
					break;
				}
				break;
			case 287:
				switch (a[i]) {
				case'\"':
					fl = 1;
					b += a[i];
					break;
				default:
					fl = 1;
					flError = 1;
					break;
				}
				break;
			case 288:
				switch (a[i]) {
				default:
					fl = 1;
					i--;
					break;
				}
				break;
			case 289:
				switch (a[i]) {
				case'\'':
					fl = 1;
					i--;
					break;
				default:
					b += a[i];
					s = 289;
					break;
				}
				break;
			case 290:
				switch (a[i]) {
				case'\'':
					fl = 1;
					b += a[i];
					break;
				default:
					fl = 1;
					flError = 1;
					break;
				}
				break;
			case 291:
				switch (a[i]) {
				case' ':
					fl = 1;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 292:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 292;
					b += a[i];
					break;
				case' ':
					fl = 1;
					i--;
					break;
				case'.':
					s = 293;
					b += a[i];
					break;
				case'e':case'E':
					s = 294;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 293:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 293;
					b += a[i];
					break;
				case' ':
					fl = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					s = 294;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 294:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 297;
					b += a[i];
					break;
				case'-':
					s = 298;
					b += a[i];
					break;
				case'+':
					s = 295;
					b += a[i];
					break;
				case' ':
					fl = 1;
					flError = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					flError = 1;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					flError = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 295:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 296;
					b += a[i];
					break;
				case' ':
					fl = 1;
					flError = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					flError = 1;
					b += a[i];
					break;
				case'-':case'+':
					flError = 1;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					flError = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 296:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 296;
					b += a[i];
					break;
				case' ':
					fl = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					flError = 1;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 297:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 297;
					b += a[i];
					break;
				case' ':
					fl = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					flError = 1;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 298:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 299;
					b += a[i];
					break;
				case' ':
					fl = 1;
					flError = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					flError = 1;
					b += a[i];
					break;
				case'-':case'+':
					flError = 1;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					flError = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 299:
				switch (a[i]) {
				case '0':case'1':case'2':case'3':case'4':case'5':case'6':case'7':case'8':case'9':
					s = 299;
					b += a[i];
					break;
				case' ':
					fl = 1;
					i--;
					break;
				case'.':
					flError = 1;
					b += a[i];
					break;
				case'e':case'E':
					flError = 1;
					b += a[i];
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || a[i] == '_') {
						s = 291;
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 300:
				switch (a[i]) {
				case'n':
					s = 301;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i]; 
					}
					break;
				}
				break;
			case 301:
				switch (a[i]) {
				case'g':
					s = 302;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 302:
				switch (a[i]) {
				case' ':
					fl = 1;
					i--;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						flError = 1;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 303:
				switch (a[i]) {
				case'a':
					s = 304;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
			case 304:
				switch (a[i]) {
				case's':
					s = 2;
					b += a[i];
					break;
				case' ':
					fl = 1;
					s = 291;
					break;
				case';':case'(':case')':case'[':case']':case'-':case'+':case':':case'.':case'!':case'*':case'&':case'/':case'%':case'<':case'>':case'=':case'^':case'|':case',':case'{':case'}':case'"':case'\'':
					fl = 1;
					s = 291;
					i--;
					break;
				default:
					if (65 <= a[i] && a[i] <= 90 || 97 <= a[i] && a[i] <= 122 || 48 <= a[i] && a[i] <= 57 || a[i] == '_') {
						s = 291;
						b += a[i];
					}
					else {
						flError = 1;
						b += a[i];
					}
					break;
				}
				break;
				
			}
			if (fl == 1) {
				bool flOfRetry = 0;
				if (flError) {
					if (s == 291) {
						textBoxOfError->Text += "ErrorInID(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					else if (s == 286 || s == 289 || s == 292 || s == 293 || s == 296 || s == 297 || s == 299 || s == 293 || s == 294) {
						textBoxOfError->Text += "ErrorInConst(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					else if (s == 3 || s == 6 || s == 12 || s == 14 || s == 17 || s == 21 || s == 25 || s == 28 || s == 31 || s == 35 || s == 39 || s == 44 || s == 46 || s == 53 || s == 57 || s == 62 || s == 66 || s == 69 || s == 78 || s == 74 || s == 83 || s == 87 || s == 89 || s == 94 || s == 96 || s == 100 || s == 105 || s == 106 || s == 110 || s == 117 || s == 126 || s == 128 || s == 130 || s == 136 || s == 144 || s == 145 || s == 152 || s == 159 || s == 164 || s == 170 || s == 175 || s == 180 || s == 184 || s == 189 || s == 193 || s == 302 || s == 198 || s == 206 || s == 209 || s == 212 || s == 215 || s == 216 || s == 221 || s == 227 || s == 231 || s == 238 || s == 241 || s == 246) {
						textBoxOfError->Text += "ErrorInKeyWord(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					else if (s == 271 || s == 274 || s == 273 || s == 276) {
						textBoxOfError->Text += "ErrorInComparisonOperators(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					else if (245 <= s && s <= 270 || 277 <= s && s <= 285 || s == 287 || s == 288 || s == 290 || s == 272 || s == 275) {
						textBoxOfError->Text += "ErrorInSymbols(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					else {}
					b = "";
					s = 0;
					fl = 0;
					flError = 0;
				}
				else {
					if (s==286||s==289||s==292||s==293 || s == 296 || s == 297 || s == 299 || s == 293) {
						if (b != "") {
							arrConst[0] = System::Convert::ToString(amountConst);
							arrConst[1] = b;
							arrConst[2] = "const" + amountConst;
							for (int g = 0; g < dataGridView2->Rows->Count; g++)
							{
								if (Convert::ToString(dataGridView2[1, g]->Value) == b)
									flOfRetry = 1;
							}
							if (flOfRetry == 0) {
								dataGridView2->Rows->Add(arrConst);
								amountConst++;
							}
							for (int g = 0; g < dataGridView2->Rows->Count; ++g)
							{
								if (Convert::ToString(dataGridView2[1, g]->Value) == b)
								{
									textBoxDesk->Text += "(" + Convert::ToString(dataGridView2->Columns[0]->Name) + "," + Convert::ToString(dataGridView2[0, g]->Value) + ") ";
									textBoxPsev->Text += Convert::ToString(dataGridView2[2, g]->Value) + " ";
								}
							}
						}
						b = "";
						if (s == 286)s = 287;
						else if (s == 289)s = 290;
						else s = 0;
						fl = 0;
					}
					else if (s == 3 || s == 6 || s == 12 || s == 14 || s == 17 || s == 21 || s == 25 || s == 28 || s == 31 || s == 35 || s == 39 || s == 44 || s == 46 || s == 53 || s == 57|| s==58 || s == 62 || s == 66 || s == 69 || s == 78 || s == 74 || s == 83 || s == 87 || s == 89 || s == 94 || s == 96 || s == 100 || s == 105 || s == 106 || s == 110 || s == 117 || s == 126 || s == 128 || s == 130 || s == 136 || s == 144 || s == 145 || s == 152 || s == 159 || s == 164 || s == 170 || s == 175 || s == 180 || s == 184 || s == 189 || s == 193 || s == 302 || s == 198 || s == 206 || s == 209 || s == 212 || s == 215 || s == 216 || s == 221 || s == 227 || s == 231 || s == 238 || s == 241 || s == 246) {
						arrWord[0] = System::Convert::ToString(amountWord);
						arrWord[1] = b;
						arrWord[2] = b;
						for (int i = 0; i < dataGridView3->Rows->Count; i++)
						{
							if (Convert::ToString(dataGridView3[1, i]->Value) == b)
								flOfRetry = 1;
						}
						if (flOfRetry == 0) {
							dataGridView3->Rows->Add(arrWord);
							amountWord++;
						}
						for (int g = 0; g < dataGridView3->Rows->Count; ++g)
						{
							if (Convert::ToString(dataGridView3[1, g]->Value) == b)
							{
								textBoxDesk->Text += "(" + Convert::ToString(dataGridView3->Columns[0]->Name) + "," + Convert::ToString(dataGridView3[0, g]->Value) + ") ";
								textBoxPsev->Text += Convert::ToString(dataGridView3[2, g]->Value) + " ";
							}
						}
						b = "";
						s = 0;
						fl = 0;
					}
					else if (s == 271 || s == 274 || s == 273 || s == 276) {
						arrOp[0] = System::Convert::ToString(amountOp);
						arrOp[1] = b;
						arrOp[2] = b;
						for (int g = 0; g < dataGridView4->Rows->Count; g++)
						{
							if (Convert::ToString(dataGridView4[1, g]->Value) == b)
								flOfRetry = 1;
						}
						if (flOfRetry == 0) {
							dataGridView4->Rows->Add(arrOp);
							amountOp++;
						}
						for (int g = 0; g < dataGridView4->Rows->Count; ++g)
						{
							if (Convert::ToString(dataGridView4[1, g]->Value) == b)
							{
								textBoxDesk->Text += "(" + Convert::ToString(dataGridView4->Columns[0]->Name) + "," + Convert::ToString(dataGridView4[0, g]->Value) + ") ";
								textBoxPsev->Text += Convert::ToString(dataGridView4[2, g]->Value) + " ";
							}
						}
						b = "";
						s = 0;
						fl = 0;
					}
					else if (245<=s&&s<=270 ||277<=s&&s<=285||s==287||s==288||s==290||s==272||s==275) {
						
							arrCh[0] = System::Convert::ToString(amountCh);
							arrCh[1] = b;
							arrCh[2] = b;
							for (int g = 0; g < dataGridView5->Rows->Count; g++)
							{
								if (Convert::ToString(dataGridView5[1, g]->Value) == b)
									flOfRetry = 1;
							}
							if (flOfRetry == 0) {
								dataGridView5->Rows->Add(arrCh);
								amountCh++;
							}
							for (int g = 0; g < dataGridView5->Rows->Count; ++g)
							{
								if (Convert::ToString(dataGridView5[1, g]->Value) == b)
								{
									textBoxDesk->Text += "(" + Convert::ToString(dataGridView5->Columns[0]->Name) + "," + Convert::ToString(dataGridView5[0, g]->Value) + ") ";
									textBoxPsev->Text += Convert::ToString(dataGridView5[2, g]->Value) + " ";
								}
							}
							b = "";
							if (s == 285)s = 286;
							else if (s == 288)s = 289;
							else s = 0;
							fl = 0;
					}
					else if (s == 291 || 1 <= s && s <= 246 || s == 303 || s == 304 || s == 300 || s == 301 || s == 302) {
						arrID[0] = System::Convert::ToString(amountID);
						arrID[1] = b;
						arrID[2] = "id" + amountID;
						for (int g = 0; g < dataGridView1->Rows->Count; ++g)
						{
							if (Convert::ToString(dataGridView1[1, g]->Value) == b)
								flOfRetry = 1;
						}
						if (flOfRetry == 0) {
							dataGridView1->Rows->Add(arrID);
							amountID++;
						}
						for (int g = 0; g < dataGridView1->Rows->Count; ++g)
						{
							if (Convert::ToString(dataGridView1[1, g]->Value) == b)
							{
								textBoxDesk->Text += "(" + Convert::ToString(dataGridView1->Columns[0]->Name) + "," + Convert::ToString(dataGridView1[0, g]->Value) + ") ";
								textBoxPsev->Text += Convert::ToString(dataGridView1[2, g]->Value) + " ";
							}
						}
						b = "";
						s = 0;
						fl = 0;
						}
					else if (s == 294 || s == 295 || s == 298) {
						if (s == 294) {
							textBoxOfError->Text += "ErrorInID(" + amountError + "): " + b;
							textBoxOfError->Text += Environment::NewLine;
						}
						else {
							textBoxOfError->Text += "ErrorInConst(" + amountError + "): " + b;
							textBoxOfError->Text += Environment::NewLine;
						}
						b = "";
						s = 0;
						fl = 0;
				}
				}
			}
			//BoxOfIdentifications->Text += s;
		}
			bool flOfRetry = 0;
			if (flError) {
				if (s == 291) {
					textBoxOfError->Text += "ErrorInID(" + amountError + "): " + b;
					textBoxOfError->Text += Environment::NewLine;
				}
				else if (s == 286 || s == 289 || s == 292 || s == 293 || s == 296 || s == 297 || s == 299 || s == 293 || s == 294) {
					textBoxOfError->Text += "ErrorInConst(" + amountError + "): " + b;
					textBoxOfError->Text += Environment::NewLine;
				}
				else if (s == 3 || s == 6 || s == 12 || s == 14 || s == 17 || s == 21 || s == 25 || s == 28 || s == 31 || s == 35 || s == 39 || s == 44 || s == 46 || s == 53 || s == 57 || s == 62 || s == 66 || s == 69 || s == 78 || s == 74 || s == 83 || s == 87 || s == 89 || s == 94 || s == 96 || s == 100 || s == 105 || s == 106 || s == 110 || s == 117 || s == 126 || s == 128 || s == 130 || s == 136 || s == 144 || s == 145 || s == 152 || s == 159 || s == 164 || s == 170 || s == 175 || s == 180 || s == 184 || s == 189 || s == 193 || s == 302 || s == 198 || s == 206 || s == 209 || s == 212 || s == 215 || s == 216 || s == 221 || s == 227 || s == 231 || s == 238 || s == 241 || s == 246) {
					textBoxOfError->Text += "ErrorInKeyWord(" + amountError + "): " + b;
					textBoxOfError->Text += Environment::NewLine;
				}
				else if (s == 271 || s == 274 || s == 273 || s == 276) {
					textBoxOfError->Text += "ErrorInComparisonOperators(" + amountError + "): " + b;
					textBoxOfError->Text += Environment::NewLine;
				}
				else if (245 <= s && s <= 270 || 277 <= s && s <= 285 || s == 287 || s == 288 || s == 290 || s == 272 || s == 275) {
					textBoxOfError->Text += "ErrorInSymbols(" + amountError + "): " + b;
					textBoxOfError->Text += Environment::NewLine;
				}
				else {}
				b = "";
				s = 0;
				fl = 0;
				flError = 0;
			}
			else {
				if (s == 286 || s == 289 || s == 292 || s == 293 || s == 296 || s == 297 || s == 299 || s == 293) {
					if (b != "") {
						arrConst[0] = System::Convert::ToString(amountConst);
						arrConst[1] = b;
						arrConst[2] = "const" + amountConst;
						for (int g = 0; g < dataGridView2->Rows->Count; g++)
						{
							if (Convert::ToString(dataGridView2[1, g]->Value) == b)
								flOfRetry = 1;
						}
						if (flOfRetry == 0) {
							dataGridView2->Rows->Add(arrConst);
							amountConst++;
						}
						for (int g = 0; g < dataGridView2->Rows->Count; ++g)
						{
							if (Convert::ToString(dataGridView2[1, g]->Value) == b)
							{
								textBoxDesk->Text += "(" + Convert::ToString(dataGridView2->Columns[0]->Name) + "," + Convert::ToString(dataGridView2[0, g]->Value) + ") ";
								textBoxPsev->Text += Convert::ToString(dataGridView2[2, g]->Value) + " ";
							}
						}
					}
					b = "";
					if (s == 286)s = 287;
					else if (s == 289)s = 290;
					else s = 0;
					fl = 0;
				}
				else if (s == 3 || s == 6 || s == 12 || s == 14 || s == 17 || s == 21 || s == 25 || s == 28 || s == 31 || s == 35 || s == 39 || s == 44 || s == 46 || s == 53 || s == 57 || s == 58 || s == 62 || s == 66 || s == 69 || s == 78 || s == 74 || s == 83 || s == 87 || s == 89 || s == 94 || s == 96 || s==98 || s == 100 || s == 105 || s == 106 || s == 110 || s == 117 || s == 126 || s == 128 || s == 130 || s == 136 || s == 144 || s == 145 || s == 152 || s == 159 || s == 164 || s == 170 || s == 175 || s == 180 || s == 184 || s == 189 || s == 193 || s == 302 || s == 198 || s == 206 || s == 209 || s == 212 || s == 215 || s == 216 || s == 221 || s == 227 || s == 231 || s == 238 || s == 241 || s == 246) {
					arrWord[0] = System::Convert::ToString(amountWord);
					arrWord[1] = b;
					arrWord[2] = b;
					for (int i = 0; i < dataGridView3->Rows->Count; i++)
					{
						if (Convert::ToString(dataGridView3[1, i]->Value) == b)
							flOfRetry = 1;
					}
					if (flOfRetry == 0) {
						dataGridView3->Rows->Add(arrWord);
						amountWord++;
					}
					for (int g = 0; g < dataGridView3->Rows->Count; ++g)
					{
						if (Convert::ToString(dataGridView3[1, g]->Value) == b)
						{
							textBoxDesk->Text += "(" + Convert::ToString(dataGridView3->Columns[0]->Name) + "," + Convert::ToString(dataGridView3[0, g]->Value) + ") ";
							textBoxPsev->Text += Convert::ToString(dataGridView3[2, g]->Value) + " ";
						}
					}
					b = "";
					s = 0;
					fl = 0;
				}
				else if (s == 271 || s == 274 || s == 273 || s == 276) {
					arrOp[0] = System::Convert::ToString(amountOp);
					arrOp[1] = b;
					arrOp[2] = b;
					for (int g = 0; g < dataGridView4->Rows->Count; g++)
					{
						if (Convert::ToString(dataGridView4[1, g]->Value) == b)
							flOfRetry = 1;
					}
					if (flOfRetry == 0) {
						dataGridView4->Rows->Add(arrOp);
						amountOp++;
					}
					for (int g = 0; g < dataGridView4->Rows->Count; ++g)
					{
						if (Convert::ToString(dataGridView4[1, g]->Value) == b)
						{
							textBoxDesk->Text += "(" + Convert::ToString(dataGridView4->Columns[0]->Name) + "," + Convert::ToString(dataGridView4[0, g]->Value) + ") ";
							textBoxPsev->Text += Convert::ToString(dataGridView4[2, g]->Value) + " ";
						}
					}
					b = "";
					s = 0;
					fl = 0;
				}
				else if (245 <= s && s <= 270 || 277 <= s && s <= 285 || s == 287 || s == 288 || s == 290 || s == 272 || s == 275) {

					arrCh[0] = System::Convert::ToString(amountCh);
					arrCh[1] = b;
					arrCh[2] = b;
					for (int g = 0; g < dataGridView5->Rows->Count; g++)
					{
						if (Convert::ToString(dataGridView5[1, g]->Value) == b)
							flOfRetry = 1;
					}
					if (flOfRetry == 0) {
						dataGridView5->Rows->Add(arrCh);
						amountCh++;
					}
					for (int g = 0; g < dataGridView5->Rows->Count; ++g)
					{
						if (Convert::ToString(dataGridView5[1, g]->Value) == b)
						{
							textBoxDesk->Text += "(" + Convert::ToString(dataGridView5->Columns[0]->Name) + "," + Convert::ToString(dataGridView5[0, g]->Value) + ") ";
							textBoxPsev->Text += Convert::ToString(dataGridView5[2, g]->Value) + " ";
						}
					}
					b = "";
					if (s == 285)s = 286;
					else if (s == 288)s = 289;
					else s = 0;
					fl = 0;
				}
				else if (s == 291 || 1 <= s && s <= 246 || s == 303 || s == 304 || s == 300 || s == 301 || s == 302) {
					arrID[0] = System::Convert::ToString(amountID);
					arrID[1] = b;
					arrID[2] = "id" + amountID;
					for (int g = 0; g < dataGridView1->Rows->Count; ++g)
					{
						if (Convert::ToString(dataGridView1[1, g]->Value) == b)
							flOfRetry = 1;
					}
					if (flOfRetry == 0) {
						dataGridView1->Rows->Add(arrID);
						amountID++;
					}
					for (int g = 0; g < dataGridView1->Rows->Count; ++g)
					{
						if (Convert::ToString(dataGridView1[1, g]->Value) == b)
						{
							textBoxDesk->Text += "(" + Convert::ToString(dataGridView1->Columns[0]->Name) + "," + Convert::ToString(dataGridView1[0, g]->Value) + ") ";
							textBoxPsev->Text += Convert::ToString(dataGridView1[2, g]->Value) + " ";
						}
					}
					b = "";
					s = 0;
					fl = 0;
				}
				else if (s == 294 || s == 295 || s == 298) {
					if (s == 294) {
						textBoxOfError->Text += "ErrorInID(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					else {
						textBoxOfError->Text += "ErrorInConst(" + amountError + "): " + b;
						textBoxOfError->Text += Environment::NewLine;
					}
					b = "";
					s = 0;
					fl = 0;
				}
			}
		amountError++;
	}
}
private: System::Void MyForm_Activated(System::Object^ sender, System::EventArgs^ e) {

	dataGridView1->ColumnCount = 3;
	dataGridView1->Columns[0]->Name = "10";
	dataGridView1->Columns[1]->Name = "Иден";
	dataGridView1->Columns[2]->Name = "код";

	dataGridView2->ColumnCount = 3;
	dataGridView2->Columns[0]->Name = "20";
	dataGridView2->Columns[1]->Name = "Конст";
	dataGridView2->Columns[2]->Name = "код";

	dataGridView3->ColumnCount = 3;
	dataGridView3->Columns[0]->Name = "30";
	dataGridView3->Columns[1]->Name = "Ключевые слова";
	dataGridView3->Columns[2]->Name = "код";

	dataGridView4->ColumnCount = 3;
	dataGridView4->Columns[0]->Name = "40";
	dataGridView4->Columns[1]->Name = "Операторы отношений";
	dataGridView4->Columns[2]->Name = "код";

	dataGridView5->ColumnCount = 3;
	dataGridView5->Columns[0]->Name = "50";
	dataGridView5->Columns[1]->Name = "Знаки";
	dataGridView5->Columns[2]->Name = "код";
}
};
}
