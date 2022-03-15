import React from 'react';
import { Avatar, Button, List } from 'antd';
import {
  FileZipTwoTone,
  FileMarkdownTwoTone,
  FolderOpenTwoTone,
  FilePdfTwoTone,
  FileUnknownTwoTone,
  FileTextTwoTone,
} from '@ant-design/icons';
import qs from 'qs';

interface Data {
  list: {
    name: string;
    path_uri: string;
    ext: string;
    is_file: boolean;
    last_modified: string;
  }[];
}

const apiUrl = process.env.REACT_APP_API_URL;

const get_icon = (is_file: boolean, ext: string) => {
  if (is_file) {
    if (ext.toLowerCase() === 'md') {
      return <FileMarkdownTwoTone />;
    } else if (ext.toLowerCase() === 'pdf') {
      return <FilePdfTwoTone />;
    } else if (ext.toLowerCase() === 'zip') {
      return <FileZipTwoTone />;
    } else if (ext.toLowerCase() === 'txt') {
      return <FileTextTwoTone />;
    } else {
      return <FileUnknownTwoTone />;
    }
  }

  return <FolderOpenTwoTone />;
};

export const PathList = ({ list }: Data) => {
  return (
    <List
      bordered={true}
      itemLayout="horizontal"
      dataSource={list}
      renderItem={(item) => {
        return (
          <List.Item>
            <List.Item.Meta
              avatar={<Avatar icon={get_icon(item.is_file, item.ext)} />}
              title={
                <Button
                  type="link"
                  onClick={() => {
                    window
                      .fetch(`${apiUrl}/file?${qs.stringify(item)}`)
                      .then(async (response) => {
                        response.blob().then((blob) => {
                          let url = window.URL.createObjectURL(blob);
                          let a = document.createElement('a');
                          a.href = url;
                          a.download = item.name;
                          a.click();
                        });
                      });
                  }}
                >
                  {item.name}
                </Button>
              }
              description={item.last_modified}
            />
          </List.Item>
        );
      }}
    />
  );
};
